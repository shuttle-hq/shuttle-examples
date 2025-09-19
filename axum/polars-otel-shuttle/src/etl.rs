use polars::prelude::*;
use std::collections::HashMap;
use std::time::Instant;
use tracing::{info, instrument};

fn rss_mb() -> f64 {
    if let Ok(s) = std::fs::read_to_string("/proc/self/status") {
        for line in s.lines() {
            if let Some(val) = line.strip_prefix("VmRSS:") {
                let kb: f64 = val.split_whitespace().nth(0).unwrap_or("0").parse().unwrap_or(0.0);
                return kb / 1024.0;
            }
        }
    }
    0.0
}

fn bump_peak(metrics: &mut HashMap<String, f64>, label: &str) {
    let m = rss_mb();
    metrics.insert(format!("{}_memory_mb", label), m);
    let peak = metrics.get("peak_memory_mb").cloned().unwrap_or(0.0);
    if m > peak {
        metrics.insert("peak_memory_mb".into(), m);
    }
}

pub struct PolarsETL {
    df: Option<LazyFrame>,
    metrics: HashMap<String, f64>,
}

impl PolarsETL {
    pub fn new() -> Self {
        Self { df: None, metrics: HashMap::new() }
    }

    pub fn get_metrics(&self) -> &HashMap<String, f64> {
        &self.metrics
    }

    #[instrument(level = "info", skip(self))]
    pub fn load_data(&mut self, file_path: &str) -> PolarsResult<&mut Self> {
        info!("Loading data: {file_path}");
        let start = Instant::now();

        let lf = LazyCsvReader::new(file_path)
            .with_has_header(true)
            .with_infer_schema_length(Some(2000))
            .map_parse_options(|opts| opts.with_try_parse_dates(false))
            .finish()?
            .select([
                col("pickup_longitude"),
                col("pickup_latitude"),
                col("dropoff_longitude"),
                col("dropoff_latitude"),
                col("trip_distance"),
                col("passenger_count"),
                col("tpep_pickup_datetime"),
                col("tpep_dropoff_datetime"),
                col("total_amount"),
            ]);

        self.df = Some(lf);
        let t = start.elapsed().as_secs_f64();
        self.metrics.insert("load_time".into(), t);
        bump_peak(&mut self.metrics, "after_load");
        info!(load_time_s = t, peak_mb = self.metrics.get("peak_memory_mb").cloned().unwrap_or(0.0), "Data scan created");
        Ok(self)
    }

    #[instrument(level = "info", skip(self))]
    pub fn clean_data(&mut self) -> PolarsResult<&mut Self> {
        info!("Cleaning data");
        let start = Instant::now();

        if let Some(df) = &self.df {
            let fmt: PlSmallStr = "%Y-%m-%d %H:%M:%S%.f".into();
            let to_dt_opts = StrptimeOptions {
                format: Some(fmt),
                strict: false,
                exact: false,
                cache: true,
                ..Default::default()
            };

            let cleaned = df
                .clone()
                .filter(
                    col("pickup_longitude").neq(lit(0.0))
                        .and(col("pickup_latitude").neq(lit(0.0)))
                        .and(col("dropoff_longitude").neq(lit(0.0)))
                        .and(col("dropoff_latitude").neq(lit(0.0)))
                        .and(col("trip_distance").gt(lit(0.0)))
                        .and(col("trip_distance").lt(lit(100.0)))
                        .and(col("passenger_count").gt(lit(0)))
                        .and(col("passenger_count").lt_eq(lit(6))),
                )
                .with_columns([
                    col("tpep_pickup_datetime").str().strptime(
                        DataType::Datetime(TimeUnit::Microseconds, None),
                        to_dt_opts.clone(),
                        lit("coerce"),
                    ),
                    col("tpep_dropoff_datetime").str().strptime(
                        DataType::Datetime(TimeUnit::Microseconds, None),
                        to_dt_opts,
                        lit("coerce"),
                    ),
                ])
                .with_columns([
                    (col("tpep_dropoff_datetime") - col("tpep_pickup_datetime"))
                        .dt()
                        .total_minutes()
                        .alias("trip_duration_minutes"),
                ])
                .filter(col("trip_duration_minutes").gt(lit(0)).and(col("trip_duration_minutes").lt(lit(480))))
                .cache();

            self.df = Some(cleaned);
        }

        let t = start.elapsed().as_secs_f64();
        self.metrics.insert("clean_time".into(), t);
        bump_peak(&mut self.metrics, "after_clean");
        info!(clean_time_s = t, peak_mb = self.metrics.get("peak_memory_mb").cloned().unwrap_or(0.0), "Cleaned");
        Ok(self)
    }

    #[instrument(level = "info", skip(self))]
    pub fn aggregate_data(&mut self) -> PolarsResult<&mut Self> {
        info!("Aggregating");
        let start = Instant::now();

        if let Some(df) = &self.df {
            let df_feats = df.clone().with_columns([
                col("tpep_pickup_datetime").dt().date().alias("date"),
                col("tpep_pickup_datetime").dt().hour().alias("hour"),
                col("tpep_pickup_datetime").dt().weekday().alias("weekday"),
            ]);

            let _daily = df_feats
                .clone()
                .group_by([col("date")])
                .agg([
                    col("trip_distance").count().alias("trip_count"),
                    col("trip_distance").mean().alias("avg_trip_distance"),
                    col("trip_distance").sum().alias("total_trip_distance"),
                    col("trip_duration_minutes").mean().alias("avg_trip_duration"),
                    col("trip_duration_minutes").sum().alias("total_trip_duration"),
                    col("passenger_count").sum().alias("total_passengers"),
                    col("total_amount").mean().alias("avg_total_amount"),
                    col("total_amount").sum().alias("total_revenue"),
                ])
                .collect()?;

            let _hourly = df_feats
                .clone()
                .group_by([col("hour")])
                .agg([
                    col("trip_distance").count().alias("trip_count"),
                    col("trip_distance").mean().alias("avg_trip_distance"),
                    col("trip_duration_minutes").mean().alias("avg_trip_duration"),
                    col("total_amount").mean().alias("avg_total_amount"),
                ])
                .collect()?;

            let _dow = df_feats
                .clone()
                .group_by([col("weekday")])
                .agg([
                    col("trip_distance").count().alias("trip_count"),
                    col("trip_distance").mean().alias("avg_trip_distance"),
                    col("total_amount").mean().alias("avg_total_amount"),
                ])
                .collect()?;

            let _ = (_daily, _hourly, _dow);
        }

        let t = start.elapsed().as_secs_f64();
        self.metrics.insert("aggregate_time".into(), t);
        bump_peak(&mut self.metrics, "after_aggregate");
        info!(aggregate_time_s = t, peak_mb = self.metrics.get("peak_memory_mb").cloned().unwrap_or(0.0), "Aggregations done");
        Ok(self)
    }

    #[instrument(level = "info", skip(self))]
    pub fn sort_and_filter(&mut self) -> PolarsResult<&mut Self> {
        info!("Sort & filter counts");
        let start = Instant::now();

        if let Some(df) = &self.df {
            let counts = df
                .clone()
                .with_columns([
                    col("tpep_pickup_datetime").dt().hour().alias("hour"),
                    col("tpep_pickup_datetime").dt().weekday().alias("weekday"),
                ])
                .select([
                    col("trip_distance").count().cast(DataType::Int64).alias("rows_after_cleaning"),
                    col("trip_distance").gt(lit(10.0)).cast(DataType::Int64).sum().alias("long_trips_count"),
                    col("total_amount").gt(lit(50.0)).cast(DataType::Int64).sum().alias("expensive_trips_count"),
                    (col("hour").eq(lit(7)).or(col("hour").eq(lit(8))).or(col("hour").eq(lit(9)))
                        .or(col("hour").eq(lit(17))).or(col("hour").eq(lit(18))).or(col("hour").eq(lit(19))))
                        .cast(DataType::Int64).sum().alias("rush_hour_trips_count"),
                    col("weekday").gt_eq(lit(6)).cast(DataType::Int64).sum().alias("weekend_trips_count"),
                    (col("trip_distance").gt(lit(5.0))
                        .and(col("total_amount").gt(lit(30.0)))
                        .and(col("passenger_count").gt_eq(lit(2))))
                        .cast(DataType::Int64).sum().alias("premium_trips_count"),
                ])
                .collect()?;

            let get_i64 = |name: &str| -> PolarsResult<i64> {
                Ok(counts.column(name)?.i64()?.get(0).unwrap_or(0))
            };

            self.metrics.insert("rows_after_cleaning".into(), get_i64("rows_after_cleaning")? as f64);
            self.metrics.insert("long_trips_count".into(),     get_i64("long_trips_count")? as f64);
            self.metrics.insert("expensive_trips_count".into(), get_i64("expensive_trips_count")? as f64);
            self.metrics.insert("rush_hour_trips_count".into(), get_i64("rush_hour_trips_count")? as f64);
            self.metrics.insert("weekend_trips_count".into(),   get_i64("weekend_trips_count")? as f64);
            self.metrics.insert("premium_trips_count".into(),   get_i64("premium_trips_count")? as f64);

            info!(
                long = self.metrics["long_trips_count"],
                expensive = self.metrics["expensive_trips_count"],
                "Counts computed"
            );
        }

        let t = start.elapsed().as_secs_f64();
        self.metrics.insert("sort_filter_time".into(), t);
        bump_peak(&mut self.metrics, "after_sort_filter");
        info!(sort_filter_time_s = t, peak_mb = self.metrics.get("peak_memory_mb").cloned().unwrap_or(0.0), "Sort & filter done");
        Ok(self)
    }

    #[instrument(level = "info", skip(self))]
    pub fn save_results(&mut self, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
        let start = Instant::now();
        std::fs::create_dir_all(output_dir)?;
        let metrics_json = serde_json::to_string_pretty(&self.metrics)?;
        std::fs::write(format!("{}/polars_metrics.json", output_dir), metrics_json)?;
        let t = start.elapsed().as_secs_f64();
        self.metrics.insert("save_time".into(), t);
        bump_peak(&mut self.metrics, "after_save");
        info!(save_time_s = t, "Saved results");
        Ok(())
    }
}

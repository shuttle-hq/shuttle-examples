<template>
  <div className="w-full h-96 shadow-md p-6 mb-10 rounded-md bg-white">
    <Line
      id="my-chart-id"
      :options="chartOptions"
      :data="chartData"
    />
  </div>
</template>

<script lang="ts">
import { Line } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, LineElement, PointElement, CategoryScale, LinearScale)

export default {
  name: 'SalesChart',
  components: { Line },
  data() {
    return {
      chartData: {
        labels: ['January', 'February', 'March', 'April', 'May', 'June', 'July'],
        datasets: [
          {
            label: 'Sales - Last 7 Months',
            data: [100, 300, 200, 100, 200, 400, 300],
            borderColor: '#EF924C',
            backgroundColor: '#EF924C',
            color: '#EF924C',
            tension: 0.3,
          },
        ],
      },
      chartOptions: {
        responsive: true,
        maintainAspectRatio: false,
        scales: {
          y: {
            min: 0,
            stepSize: 100,
            max: 500,
            ticks: {
              stepSize: 100,
            },
          },
        },
        plugins: {
          title: {
            display: false,
          },
        },
      }
    }
  }
}
</script>

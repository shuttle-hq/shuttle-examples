About the Project



This project is an API server developed using the Rust programming language. It leverages the Actix Web framework 

for handling HTTP-based requests and integrates with the Turso database for performing various data operations.



Technologies Used

- Actix Web: A fast and powerful web framework.

- Actix Session: Used for session management.

- Turso Database: A fast and remote database solution.

- dotenv: For managing environment variables easily.

- Shuttle Runtime: A cloud solution for quickly deploying and running the project.



API Endpoints

| Method | Endpoint           | Description                |

|--------|--------------------|----------------------------|

| POST   | /api/user/create   | Creates a new user.        |

| POST   | /api/user/login    | Handles user login.        |

| POST   | /api/news/upload   | Uploads news.              |

| POST   | /api/events/upload | Uploads events.            |

| GET    | /api/get           | Fetches data.              |

| GET    | /api/get/a         | Fetches statistics.        |

| POST   | /api/check/token   | Verifies a token.          |



Setting Up the Development Environment



1. Clone the Repository:

   git clone https://github.com/your-username/your-repository.git

   cd your-repository



2. Install Required Dependencies:

   Use Cargo to install the dependencies:

   cargo build



3. Configure Environment Variables:

   Add the following keys to your Secrets.toml file:

   TURSO_DATABASE_URL=<database-url>

   TURSO_AUTH_TOKEN=<auth-token>



4. Run the Project:

   cargo run



Deploying the Project with Shuttle



1. Install Shuttle CLI:

   cargo install cargo-shuttle



2. Deploy the Project:

   cargo shuttle deploy

"""

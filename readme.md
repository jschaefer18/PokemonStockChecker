## Overview

**StockTracker**:

**Project Description**:
This project is a simple full-stack stock market tracker that alolws the user to search for real-time stock data using a ticker symbol. The project consists of two main components: a backend server built with Rust and Axum, and a frontend user interface built with React and Vite. The backend server fetches stock data from the Finnhub API and serves it to the frontend, which displays the information in a user-friendly format.

**Project Goals**:
- Build a clean and functional user interface to query stock data.
- Connect a Rust backend to a financial data API.
- Learn about full-stack development with Rust and React.
- Gain experience with Axum, Reqwest, Serde, and Chrono in Rust.
- Handle CORS issues for frontend-backend communication.


## Instructions for Build and Use

Steps to build and/or run the software:

1. Clone the repository and navigate to the project directory.
2. run `cargo run` in the backend directory to start the Rust server.
3. Open a new terminal and navigate to the frontend directory.
4. run `npm install` to install the necessary dependencies.
5. run `npm run dev` to start the React frontend.
6. Open your web browser and navigate to `http://localhost:5173` to access the application.

Instructions for using the software:

1. Open your web browser and navigate to `http://localhost:5173`.
2. Enter a stock ticker symbol (e.g., AAPL) in the input field.
3. Click "Search" to fetch and view live stock data.



## Development Environment 

To recreate the development environment, you need the following software and/or libraries with the specified versions:

* Rust (stable, with Cargo)
* Node.js (v18 or later)
* NPM (v9 or later)
* Finnhub API key (free key available at finnhub.io)
* Vite (comes with React template via npm create vite@latest)


## Useful Websites to Learn More

I found these websites useful in developing this software:

* Finnhub API Docs: https://finnhub.io/docs/api/quote
* Axum Rust Framework: http://docs.rs/axum/latest/axum/
* Vite + React Docs: https://vite.dev/guide/
* MDN Web Docs: https://developer.mozilla.org/en-US/docs/Web/API/Fetch_API
* Chrono Rust Docs: https://docs.rs/chrono/latest/chrono/
* Personalized Rust ChatGPT Assistant: https://chat.openai.com/chat
* Rust Programming Language Book: https://doc.rust-lang.org/book/
* React Documentation: https://reactjs.org/docs/getting-started.html

## Future Work

The following items I plan to fix, improve, and/or add to this project in the future:

* [ ] I want to add error handling for invalid ticker symbols.
* [ ] I want to improve the UI design and layout for better user experience. Potentially adding a light mode/dark mode toggle.
* [ ] Add support for viewing historical stock data.
* [ ] Finally, this is very ambitious, but I want to deploy the frontend and backend to a cloud service (e.g., AWS, Heroku) for public access.

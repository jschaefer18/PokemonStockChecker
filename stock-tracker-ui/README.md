# React + Vite

This template provides a minimal setup to get React working in Vite with HMR and some ESLint rules.

Currently, two official plugins are available:

- [@vitejs/plugin-react](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react) uses [Babel](https://babeljs.io/) for Fast Refresh
- [@vitejs/plugin-react-swc](https://github.com/vitejs/vite-plugin-react/blob/main/packages/plugin-react-swc) uses [SWC](https://swc.rs/) for Fast Refresh

## Expanding the ESLint configuration

If you are developing a production application, we recommend using TypeScript with type-aware lint rules enabled. Check out the [TS template](https://github.com/vitejs/vite/tree/main/packages/create-vite/template-react-ts) for information on how to integrate TypeScript and [`typescript-eslint`](https://typescript-eslint.io) in your project.

Purpose: Acts as a server that takes a stock ticker via query string and returns stock data using the Finnhub API.

Key Components:

Axum for routing (GET /quote?ticker=AAPL)

Reqwest for HTTP requests

Serde for JSON serialization/deserialization

Chrono for timestamp formatting

Cross-Origin: CORS is enabled with CorsLayer::permissive() to allow your frontend to communicate with it.

Server URL: http://127.0.0.1:3000/quote?ticker=AAPL

💻 Frontend (React with Vite)
Purpose: Simple UI that accepts a stock ticker, fetches data from the backend, and displays the stock information.

Current Status:

React app loads

Ticker input + submit fetches real-time data

Displays price, percent change, high/low, market timestamp

Centering not perfect (but you’ve opted to move forward)

✅ Working Now
Typing a ticker like AAPL shows correct stock info

API requests reach backend and return parsed JSON

Backend and frontend communicate correctly

Time is localized, request and market times shown

🧠 Known Issues / Next Up
Centering the frontend still a little off

Time only updates if the data on the API side updates (out of your control)

You may eventually want better error handling or input validation



cargo run

cd stock-tracker-ui

npm run dev

// This is the main component of the app where the UI logic lives. It manages the search input, handles API requests to the backend, and displays the resulting stock data. 
// It’s the core of the frontend user experience.



import React, { useState } from 'react';
import './index.css';

function App() {
  const [ticker, setTicker] = useState('');
  const [data, setData] = useState(null);

  const fetchQuote = async () => {
    const res = await fetch(`http://localhost:3000/quote?ticker=${ticker}`);
    const json = await res.json();
    setData(json);
  };

  return (
    <div className="App">
      <h1>Stock Tracker</h1>
      <input
        type="text"
        value={ticker}
        onChange={(e) => setTicker(e.target.value)}
        placeholder="Enter ticker"
      />
      <button onClick={fetchQuote}>Search</button>

      {data && (
        <div className="card">
          <p><strong>Ticker:</strong> {data.ticker}</p>
          <p><strong>Price:</strong> ${data.current_price}</p>
          <p><strong>Change:</strong> ${data.change} ({data.percent_change}%)</p>
          <p><strong>High:</strong> ${data.high} | <strong>Low:</strong> ${data.low}</p>
          <p><strong>Market Time:</strong> {data.market_time}</p>
          <p><strong>Request Time:</strong> {data.request_time}</p>
        </div>
      )}
    </div>
  );
}

export default App;

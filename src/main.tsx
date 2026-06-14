import React from "react";
import ReactDOM from "react-dom/client";
import App from "./App";
import "./styles.css";
import { api } from "./services/api";

async function bootstrap() {
  try {
    await api.initDb();
  } catch (e) {
    console.error("[bootstrap] DB init failed:", e);
  }

  ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
    <React.StrictMode>
      <App />
    </React.StrictMode>
  );
}

bootstrap();
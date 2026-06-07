import React from "react";
import {
  Chart as ChartJS,
  CategoryScale,
  LinearScale,
  BarElement
} from "chart.js";
import { Bar } from "react-chartjs-2";

ChartJS.register(CategoryScale, LinearScale, BarElement);

export default function ChartView() {
  const data = {
    labels: ["Agents", "Reports"],
    datasets: [
      {
        label: "System Stats",
        data: [5, 20],
        backgroundColor: ["#22c55e", "#3b82f6"]
      }
    ]
  };

  return (
    <div style={box}>
      <h2>📊 Overview</h2>
      <Bar data={data} />
    </div>
  );
}

const box = {
  background: "#020617",
  marginTop: 20,
  padding: 20,
  borderRadius: 10
};

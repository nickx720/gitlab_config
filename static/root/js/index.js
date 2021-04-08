"use strict";
const loadContent = () => {
  const content = document.getElementsByTagName("body")[0];
  const darkMode = document.getElementById("dark-change");

  darkMode.addEventListener("click", function () {
    darkMode.classList.toggle("active");
    content.classList.toggle("night");
  });
};

const random_rgba = () => {
  const limit = 255;
  return (
    "rgba(" +
    Math.round(Math.random() * limit) +
    "," +
    Math.round(Math.random() * limit) +
    "," +
    Math.round(Math.random() * limit) +
    "," +
    Math.random().toFixed(1) +
    ")"
  );
};

const createChart = ({ labels, data }) => {
  const rgbSpectrum = Array.from({ length: labels.length }, () =>
    random_rgba()
  );
  const ctx = document.getElementById("myChart").getContext("2d");
  const myChart = new Chart(ctx, {
    type: "doughnut",
    data: {
      labels,
      datasets: [
        {
          label: "BreakDown Of Languages",
          data,
          backgroundColor: rgbSpectrum,
          borderColor: rgbSpectrum,
          borderWidth: 1,
        },
      ],
    },
    options: {
      responsive: true,
      aspectRatio: 1,
    },
  });
};

const progressBar = (progressBlock) => {
  const progress = document.getElementById("file");
  function frame() {
    if (progress.value === 100) {
      clearInterval(timer);
      progressBlock.style.display = "None";
    }
    progress.value += 0.1;
  }
  const timer = setInterval(frame, 1);
  return timer;
};

const fetchData = async () => {
  try {
    const progressBlock = document.getElementsByClassName("progress")[0];
    const timer = progressBar(progressBlock);
    const response = await fetch("./summary");
    const json = await response.json();
    const total = json.length;
    const data = new Map();
    for (const item of json) {
      for (const [key, value] of Object.entries(item)) {
        if (data.has(key)) {
          const updatedValue = data.get(key) + Number(value);
          data.set(key, updatedValue);
        } else {
          data.set(key, Number(value));
        }
      }
    }
    for (const [key, value] of data) {
      data.set(key, Number.parseFloat(value / total).toFixed(2));
    }
    const chart = {
      labels: [...data.keys()],
      data: [...data.values()],
    };
    createChart(chart);
    clearInterval(timer);
    progressBlock.style.display = "None";
  } catch (e) {
    console.log("Something went wrong " + e.message);
  }
};

const main = () => {
  loadContent();
  fetchData();
};

main();

const { invoke } = window.__TAURI__.tauri;

let dayInputEl = document.getElementById('day');
let monthInputEl = document.getElementById('month');
let yearInputEl = document.getElementById('year');
let resultEl = document.getElementById('result');

async function get_numerology() {
    const res = await invoke("calculate", {
        day: Number(dayInputEl.value),
        month: Number(monthInputEl.value),
        year: Number(yearInputEl.value),
    })
    .catch(error => console.error(error));
    // resultEl.textContent = JSON.stringify(res, null, "\t");
    resultEl.innerText = JSON.stringify(res, ["eins", "zwei", "drei", "vier", "fuenf", "sechs", "sieben", "acht", "mitte"], 4);
    console.log(res);
}

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#calculate").addEventListener("click", (e) => {
        e.preventDefault();
        resultEl.innerHTML = "Calculating ...";
        get_numerology();
    });
})

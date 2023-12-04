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
    // resultEl.innerHTML = JSON.stringify(res, ["eins", "zwei", "drei", "vier", "fuenf", "sechs", "sieben", "acht", "mitte"], 4);
    console.log(res);
    document.getElementById("resEins").textContent = res.eins;
    document.getElementById("resZwei").textContent = res.zwei;
    document.getElementById("resDrei").textContent = res.drei;
    document.getElementById("resVier").textContent = res.vier;
    document.getElementById("resFuenf").textContent = res.fuenf;
    document.getElementById("resSechs").textContent = res.sechs;
    document.getElementById("resSieben").textContent = res.sieben;
    document.getElementById("resAcht").textContent = res.acht;
    document.getElementById("resMitte").textContent = res.mitte;

}

function clear_all () {
    dayInputEl.value = "";
    monthInputEl.value = "";
    yearInputEl.value = "";
    //resultEl.textContent = "";
    document.getElementById("resEins").textContent = "";
    document.getElementById("resZwei").textContent = "";
    document.getElementById("resDrei").textContent = "";
    document.getElementById("resVier").textContent = "";
    document.getElementById("resFuenf").textContent = "";
    document.getElementById("resSechs").textContent = "";
    document.getElementById("resSieben").textContent = "";
    document.getElementById("resAcht").textContent = "";
    document.getElementById("resMitte").textContent = "";
}

window.addEventListener("DOMContentLoaded", () => {
    document.querySelector("#calculate").addEventListener("click", (e) => {
        e.preventDefault();
        // resultEl.innerHTML = "Calculating ...";
        get_numerology();
        // resultEl.innerHTML = "Done!";
    });
    document.querySelector("#clear").addEventListener("click", (e) => {
        e.preventDefault();
        // resultEl.innerHTML = "Calculating ...";
        clear_all();
        // resultEl.innerHTML = "Done!";
    });
})

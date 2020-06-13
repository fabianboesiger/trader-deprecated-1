document.addEventListener("DOMContentLoaded", event => {
    let uri = "ws://" + location.host + "/socket";
    let ws = new WebSocket(uri);
    let investments = document.getElementById("investments");
    let loading = document.getElementById("loading");
    let stats = document.getElementById("stats");

    ws.onopen = function() {
        console.log("connected");
    }

    ws.onmessage = function(message) {
        stats.style.display = "block";
        loading.style.display = "none";

        let data = JSON.parse(message.data);
        let balances = data.balances;

        updateElement(document.getElementById("total"), data.total.toFixed(2), true);
        updateElement(document.getElementById("duration"), (data.secondsRunning / 60.0 / 60.0 / 24.0).toFixed(1));
        updateElement(document.getElementById("profit"), (data.profitPerDay * 100.0).toFixed(2));

        let assetRows = document.getElementsByClassName("asset-row");
        for (let i = 0; i < assetRows.length; i++) {
            assetRows[i].classList.add("untouched");
        }
        for (let balance of balances) {
            let element = document.getElementById(balance.symbol);
            let roundedBalance = balance.balance.toFixed(8);
            let roundedBalanceUSDT = balance.usdt.toFixed(2);
            if (element !== null) {
                if (balance.balance !== 0.0) {
                    element.classList.remove("untouched");
                    let balanceElement = element.getElementsByClassName("balance")[0];
                    updateElement(balanceElement, roundedBalance);
                    let balanceElementUSDT = element.getElementsByClassName("balance-usdt")[0];
                    updateElement(balanceElementUSDT, roundedBalanceUSDT, true);
                } else {
                    element.outerHTML = "";
                }
            } else {
                if (balance.balance !== 0.0) {
                    investments.innerHTML += ("<tr class=\"asset-row\" id=\"" + balance.symbol + "\"><td class=\"asset\">" + balance.symbol + "</td><td class=\"balance\">" + roundedBalance + "</td><td class=\"balance-usdt\">" + roundedBalanceUSDT + "</td></tr>");
                }
            }
        }
        let untoucheds = document.getElementsByClassName("untouched");
        for (let i = 0; i < untoucheds.length; i++) {
            untoucheds[i].outerHTML = "";
        }

    };
});


function updateElement(element, update, color = false) {
    element
    if (element.innerHTML != update) {
        if (element.innerHTML !== "" && color) {
            if (element.innerHTML < update) {
                element.classList.remove("loss");
                element.classList.add("win");
            } else
            if (element.innerHTML > update) {
                element.classList.remove("win");
                element.classList.add("loss");
            }
        }
        element.innerHTML = update;
    } else {
        //element.classList.remove("win");
        //element.classList.remove("loss");
    }
}
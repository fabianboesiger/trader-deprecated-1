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

        // Update total.
        let roundedTotal = data.total.toFixed(2);
        let totalElement = document.getElementById("total");
        if (totalElement.innerHTML != roundedTotal) {
            totalElement.innerHTML = roundedTotal;
        }

        let roundedDays = (data.secondsRunning / 60.0 / 60.0 / 24.0).toFixed(1);
        let durationElement = document.getElementById("duration");
        if (durationElement.innerHTML != roundedDays) {
            durationElement.innerHTML = roundedDays;
        }

        let roundedProfit = (data.profitPerDay * 100.0).toFixed(2);
        let profitElement = document.getElementById("profit");
        if (profitElement.innerHTML != roundedProfit) {
            profitElement.innerHTML = roundedProfit;
        }

        // Update investments.
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
                    if (balanceElement.innerHTML != roundedBalance) {
                        balanceElement.innerHTML = roundedBalance;
                    }
                    let balanceElementUSDT = element.getElementsByClassName("balance-usdt")[0];
                    if (balanceElementUSDT.innerHTML != roundedBalanceUSDT) {
                        balanceElementUSDT.innerHTML = roundedBalanceUSDT;
                    }
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
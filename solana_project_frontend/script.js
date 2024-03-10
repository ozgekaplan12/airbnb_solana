async function listHouse() {
    const owner = document.getElementById('owner').value;
    const price = document.getElementById('price').value;

    // Call Solana backend to list the house
    const response = await fetch('/list-house', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ owner, price })
    });

    const data = await response.json();
    document.getElementById('message').innerText = data.message;
}

async function reserveHouse() {
    const guest = document.getElementById('guest').value;
    const house = document.getElementById('house').value;

    // Call Solana backend to reserve the house
    const response = await fetch('/reserve-house', {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json'
        },
        body: JSON.stringify({ guest, house })
    });

    const data = await response.json();
    document.getElementById('message').innerText = data.message;
}

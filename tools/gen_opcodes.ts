
const to_hex = (d) => ('00' + (+d).toString(16)).slice(-2);


for (let i = 0; i <= 0xFF; i++) {
    console.log(`None, // 0x${to_hex(i)}`);
}



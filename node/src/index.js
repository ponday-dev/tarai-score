function tarai(x, y, z) {
    if (x <= y) {
        return y;
    }
    return tarai(
        tarai(x - 1, y, z),
        tarai(y - 1, z, x),
        tarai(z - 1, x, y)
    );
}

const start = new Date();
const result = tarai(15, 5, 0);
const end = new Date();

console.log(result);
console.log(`${(end - start) / 1000}s`);


const typescript = require("@rollup/plugin-typescript");

const production = process.env.PRODUCTION_BUILD;

export default {
    input: "src/DoggyBag.ts",
    output: {
        dir: "dist",
        format: "cjs",
        sourcemap: !production
    },
    plugins: [
        typescript()
    ]
};
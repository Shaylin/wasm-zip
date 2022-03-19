module.exports = {
    collectCoverage: true,
    coverageReporters: ["text", "text-summary"],
    preset: "ts-jest",
    testEnvironment: "jsdom",
    "testMatch": ["**/test/**/*Spec.ts"]
}

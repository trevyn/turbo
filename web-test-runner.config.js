const vite = require("vite-web-test-runner-plugin");

module.exports = {
 plugins: [vite()],
 browserStartTimeout: 120000,
};

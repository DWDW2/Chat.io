require("@babel/register")({
  presets: ["@babel/preset-env", "@babel/preset-react"],
});

const React = require("react");
const ReactDOMServer = require("react-dom/server");
const App = require("./application").default; // Import your App component

const html = ReactDOMServer.renderToString(React.createElement(App));

console.log(html);

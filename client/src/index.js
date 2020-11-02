import React from "react";
import ReactDOM from "react-dom";
import "./index.css";
import App from "./App";
import { BrowserRouter } from "react-router-dom";
// Apollo
import { ApolloProvider } from "@apollo/react-hooks";
import { ApolloClient, InMemoryCache } from "@apollo/client";
import { onError } from "apollo-link-error";
const { createUploadLink } = require("apollo-upload-client");

const errorLink = onError(({ graphQLErrors, networkError }) => {
	if (graphQLErrors) {
		console.log(JSON.stringify(graphQLErrors, 0, 0));
	}
	if (networkError) console.log(`[Network error]: ${networkError}`);
});

const httpLink = createUploadLink({
	uri: "/",
	headers: {
		credentials: "same-origin",
		"content-type": "application/json",
		Authorization: window.sessionStorage.getItem("token") || "",
	},
	onError: errorLink,
});

// const link = httpLink.concat(errorLink);

const client = new ApolloClient({
	cache: new InMemoryCache(),
	link: httpLink,
});

ReactDOM.render(
	<React.StrictMode>
		<BrowserRouter>
			<ApolloProvider client={client}>
				<App />
			</ApolloProvider>
		</BrowserRouter>
	</React.StrictMode>,
	document.getElementById("root")
);

// If you want to start measuring performance in your app, pass a function
// to log results (for example: reportWebVitals(console.log))
// or send to an analytics endpoint. Learn more: https://bit.ly/CRA-vitals

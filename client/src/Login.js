import React, { useState } from "react";
import { useMutation } from "@apollo/react-hooks";
import { gql } from "@apollo/client";
import TerminalHeader from "./TermHead";

const SIGNIN = gql`
	mutation SignIn($email: String!, $password: String!) {
		signin(email: $email, password: $password)
	}
`;

export default function Login() {
	const [form, setForm] = useState({ email: "", password: "" });
	const [singIn] = useMutation(SIGNIN, {
		variables: form,
		onError: (e) => {
			console.log(e);
		},
		onCompleted: (data) => {
			if (data) {
				setForm({ email: "", password: "" });
				window.sessionStorage.setItem("token", `${data.signin}`);
				window.location.replace("/dashboard");
			}
		},
	});
	let command = "pix --login";
	return (
		<div style={{ padding: "0.5rem 1rem", lineHeight: "1.6rem" }}>
			<div className="css-typing">
				<TerminalHeader />
				<p
					style={{
						width: `${command.length - 2}em`,
						WebkitAnimation: `type ${command.length / 8}s steps(${
							command.length
						}, end)`,
						animation: `type ${command.length / 8}s steps(${command.length}, end)`,
						WebkitAnimationFillMode: "forwards",
						AnimationFillMode: "forwards",
					}}
				>
					{command}
				</p>
			</div>
			<div
				style={{
					animationName: "show",
					animationDuration: `${(command.length / 8) * 1000}ms`,
					display: "grid",
					gridTemplateColumns: "1fr",
					gridGap: "5px",
				}}
			>
				<br />
				<label>Email: </label>
				<input
					type="email"
					style={{ width: "100%", border: "2px solid green", color: "green" }}
					onChange={(e) => setForm({ ...form, email: e.target.value })}
				/>
				<br />
				<br />
				<label>Password: </label>
				<input
					type="password"
					style={{ width: "100%", border: "2px solid green", color: "green" }}
					onChange={(e) => setForm({ ...form, password: e.target.value })}
				/>
				<br />
				<br />
				<button
					style={{
						width: "10%",
						border: "2px solid green",
						color: "green",
						justifySelf: "end",
					}}
					onClick={(e) => singIn()}
				>
					Login
				</button>
			</div>
		</div>
	);
}

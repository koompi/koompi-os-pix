import React, { useState, useEffect } from "react";
import TerminalHeader from "./TermHead";
import { gql } from "@apollo/client";
import { useQuery } from "@apollo/react-hooks";
import { v4 as Uuidv4 } from "uuid";
import { CopyToClipboard } from "react-copy-to-clipboard";

const GET_APPS = gql`
	query {
		apps {
			name
			description
			maintainer
			pgpKey
			buildDate
			address
		}
	}
`;

export default function Packages() {
	const [model, setModel] = useState(false);
	const [selected, setSelected] = useState({});
	const [appState, setAppState] = useState([]);
	const { loading, error, data } = useQuery(GET_APPS, {
		onCompleted: (data) => {
			setAppState(data.apps);
		},
	});

	const toggle = () => {
		setModel(!model);
	};

	useEffect(() => {}, [appState]);
	if (loading) return null;
	if (error) return `Error! ${error}`;
	let command = "pix --list";
	let action_btn = {
		marginRight: "10px",
		border: "none",
		color: "white",
		background: "green",
		cursor: "pointer",
	};

	if (data) {
		return (
			<div style={{ padding: "0.5rem 1rem", lineHeight: "1.6rem" }}>
				<div className="css-typing">
					<TerminalHeader />
					<p
						style={{
							width: `${command.length - 1}em`,
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
				<br />
				<div
					style={{
						animationName: "show",
						animationDuration: `${(command.length / 8) * 1000}ms`,
					}}
					className="packages"
				>
					<p className="message">Showing all packages... </p>
					<br />
					<input
						className="filter"
						style={{
							width: "100%",
							height: "40px",
							border: "green 2px solid",
							color: "green",
							fontWeight: "400",
							fontSize: "1rem",
							padding: "10px",
						}}
						placeholder="Type to filter by name"
						onChange={(e) => {
							let res = data.apps.filter((app) => app.name.includes(e.target.value));
							setAppState(res);
						}}
					/>
					<div className="list-no">NO</div>
					<div className="list-name">NAME</div>
					<div className="list-install">INSTALL</div>
					<div className="list-action">ACTIONS</div>

					{appState.map((app, i) => (
						<div className="list-item" key={Uuidv4()}>
							<div className="list-no">{i + 1}</div>
							<div className="list-name">{app.name}</div>
							<div className="list-install">pix -i {app.name}</div>
							<div className="list-action">
								<CopyToClipboard text={`pix install ${app.name}`}>
									<button style={action_btn}>Copy</button>
								</CopyToClipboard>
								<button
									style={action_btn}
									onClick={(e) => {
										setSelected({ ...app });
										toggle();
									}}
								>
									Info
								</button>
								<button
									style={action_btn}
									onClick={(e) => window.open(`${app.address}`)}
								>
									Download
								</button>
							</div>
						</div>
					))}

				</div>
				{model ? <Modal data={selected} toggle={toggle} show={model} /> : ""}
			</div>
		);
	}
}

function Modal({ data, toggle, show }) {
	return (
		<div className="popup">
			<div className="modal">
				<p className="label">Name: </p>
				<p className="info">{data.name}</p>
				<p className="label">Maintainer:</p>
				<p className="info">{data.maintainer}</p>
				<p className="label">Version: </p>
				<p className="info">{data.buildDate}</p>
				<p className="label">Description: </p>
				<p className="info">{data.description}</p>
				<p className="label">PGP Signature: </p>
				<p className="info">{data.pgpKey}</p>
				<button id="modal_btn" onClick={(e) => toggle()}>
					Okay
				</button>
			</div>
		</div>
	);
}

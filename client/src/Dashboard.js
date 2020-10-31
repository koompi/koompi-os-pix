import React, { useState, Fragment, useEffect } from "react";
import { useLazyQuery, useMutation } from "@apollo/react-hooks";
import TerminalHeader from "./TermHead";
import { v4 } from "uuid";
import { gql } from "@apollo/client";

const cmd_template = { cmd: "", executed: false, res: "" };
const app_template = { name: "", description: "", maintainer: "", pgpKey: "", buildDate: "" };
const extra_ui_template = { show: false, target: "" };

const CREATE_APP = gql`
	mutation Add(
		$name: String!
		$description: String!
		$maintainer: String!
		$pgpKey: String!
		$buildDate: String!
	) {
		add(
			name: $name
			description: $description
			maintainer: $maintainer
			pgpKey: $pgpKey
			buildDate: $buildDate
		) {
			name
			description
			maintainer
			pgpKey
			buildDate
		}
	}
`;

const UPDATE_APP = gql`
	mutation Update(
		$name: String!
		$description: String!
		$maintainer: String!
		$pgpKey: String!
		$buildDate: String!
	) {
		update(
			name: $name
			description: $description
			maintainer: $maintainer
			pgpKey: $pgpKey
			buildDate: $buildDate
		) {
			name
			description
			maintainer
			pgpKey
			buildDate
		}
	}
`;

const GET_APP = gql`
	query AppByName($name: String!) {
		appByName(name: $name) {
			name
			description
			maintainer
			pgpKey
			buildDate
		}
	}
`;

export default function Dashboard() {
	const [completed, execCmds] = useState([]);
	const [new_app_state, setNewAppState] = useState(app_template);
	const [update_app_state, setUpdateAppState] = useState(app_template);
	const [current_cmd, run_cmd] = useState(cmd_template);
	const [extra_ui, setExtraUI] = useState(extra_ui_template);

	// Graphql
	const [create_app, c_data] = useMutation(CREATE_APP, {
		variables: {
			...new_app_state,
		},
	});

	const [update_app, { u_data }] = useMutation(UPDATE_APP, {
		variables: {
			...update_app_state,
		},
	});
	const [loadQuery, QueryData] = useLazyQuery(GET_APP, {
		variables: {
			name: update_app_state.name,
		},
		onCompleted: (data) => {
			let { __typename, ...others } = data.appByName;
			setUpdateAppState({ ...others });
			setExtraUI({ show: true, target: "update" });
		},
	});

	const mutationHandler = async (data) => {
		switch (data.name) {
			case "add":
				setNewAppState({ ...new_app_state, name: data.app_name });
				setExtraUI({ show: true, target: "add" });
				break;
			case "update":
				setUpdateAppState({ ...update_app_state, name: data.app_name });
				loadQuery();
				break;
			case "remove":
				break;
			default:
				console.log("command not found!");
				break;
		}
	};

	// let Extra = () => {
	// 	return (

	// 	);
	// };
	useEffect(() => {
		console.log(JSON.stringify(completed, 0, 2));
	}, []);
	return (
		<div
			style={{
				width: "100%",
				display: "grid",
				gridTemplateColumns: "200px 1fr",
				padding: "0.5rem 1rem",
				lineHeight: "1.6rem",
				color: "green",
			}}
		>
			{completed.length > 0 &&
				completed.map((cmd) => {
					console.log(cmd);
					return <Line cmd={cmd} key={v4()} />;
				})}
			<TerminalHeader />{" "}
			<input
				style={{
					color: "green",
					border: "none",
					fontSize: "1rem",
					lineHeight: "2rem",
					margin: 0,
					padding: 0,
				}}
				value={current_cmd.cmd}
				onChange={(e) => run_cmd({ ...current_cmd, cmd: e.target.value })}
				onKeyDown={(e) => {
					if (e.key === "Enter") {
						let verb = current_cmd.cmd.split(" ")[1];
						let app = current_cmd.cmd.split(" ")[2];
						mutationHandler({ name: verb, app_name: app });
					}
				}}
			/>
			<br />
			<br />
			{/* {extra_ui.show && <Extra target={extra_ui.target} />} */}
			<div
				style={{
					gridColumn: "1/3",
					minHeight: "100px",
					border: "2px solid green",
					padding: "10px",
					position: "relative",
					visibility: extra_ui.show ? "visible" : "hidden",
				}}
			>
				{extra_ui.target === "add" && <p>Create new app...</p>}
				{extra_ui.target === "update" && <p>Updaete existing app...</p>}
				<label>Name</label>
				<input
					style={{ width: "100%" }}
					value={extra_ui.target === "add" ? new_app_state.name : update_app_state.name}
					onChange={(e) =>
						extra_ui.target === "add"
							? setNewAppState({ ...new_app_state, name: e.target.value })
							: setUpdateAppState({ ...update_app_state, name: e.target.value })
					}
				/>
				<label>Description</label>
				<textarea
					style={{ width: "100%" }}
					value={
						extra_ui.target === "add"
							? new_app_state.description
							: update_app_state.description
					}
					onChange={(e) =>
						extra_ui.target === "add"
							? setNewAppState({ ...new_app_state, description: e.target.value })
							: setUpdateAppState({
									...update_app_state,
									description: e.target.value,
							  })
					}
				/>
				<label>Maintainer</label>
				<input
					style={{ width: "100%" }}
					value={
						extra_ui.target === "add"
							? new_app_state.maintainer
							: update_app_state.maintainer
					}
					onChange={(e) =>
						extra_ui.target === "add"
							? setNewAppState({ ...new_app_state, maintainer: e.target.value })
							: setUpdateAppState({ ...update_app_state, maintainer: e.target.value })
					}
				/>
				<label>Build Date</label>
				<input
					style={{ width: "100%" }}
					value={
						extra_ui.target === "add"
							? new_app_state.buildDate
							: update_app_state.buildDate
					}
					onChange={(e) =>
						extra_ui.target === "add"
							? setNewAppState({ ...new_app_state, buildDate: e.target.value })
							: setUpdateAppState({ ...update_app_state, buildDate: e.target.value })
					}
				/>
				<input type="file" id="actual-btn" name="filename" hidden />

				<label
					id="upload_btn"
					htmlFor="actual-btn"
					style={{ position: "absolute", left: 10, bottom: 10, padding: "0px 10px" }}
				>
					Upload
				</label>
				<button
					style={{ position: "absolute", right: 10, bottom: 10 }}
					onClick={(e) => {
						switch (extra_ui.target) {
							case "add":
								create_app().then((res) => {
									if (res.data.add) {
										setExtraUI({ show: false, target: "" });

										let new_cmds = completed;
										new_cmds.push({
											...current_cmd,
											res: `Succeed: ${JSON.stringify(res.data.add)}`,
										});
										execCmds([...new_cmds]);
										run_cmd(cmd_template);
									}
								});
								break;
							case "update":
								break;
							default:
								break;
						}
					}}
				>
					{extra_ui.target == "add" ? "Create" : "Update"}
				</button>
				<br />
				<br />
				<br />
			</div>
		</div>
	);
}

function Line({ cmd }) {
	return (
		<Fragment>
			<TerminalHeader /> <p style={{ margin: 0, padding: 0 }}>{cmd.cmd}</p>
			<p>{cmd.res.substring(0, 60)}...</p>
			<br />
		</Fragment>
	);
}

import React, { useState, Fragment, useEffect } from "react";
import { useLazyQuery, useMutation } from "@apollo/react-hooks";
import TerminalHeader from "./TermHead";
import { v4 } from "uuid";
import { gql } from "@apollo/client";
import { Redirect } from "react-router-dom";
// import  from ""

const cmd_template = { cmd: "", error: false, res: "" };
const app_template = {
	name: "",
	description: "",
	maintainer: "",
	pgpKey: "",
	buildDate: "",
	address: "",
};
const update_template = {
	targetName: "",
	name: "",
	description: "",
	maintainer: "",
	pgpKey: "",
	buildDate: "",
	address: "",
};
const extra_ui_template = { show: false, target: "" };
const remove_template = { name: "" };

const CREATE_APP = gql`
	mutation Add(
		$name: String!
		$description: String!
		$maintainer: String!
		$pgpKey: String!
		$buildDate: String!
		$address: String!
	) {
		add(
			name: $name
			description: $description
			maintainer: $maintainer
			pgpKey: $pgpKey
			buildDate: $buildDate
			address: $address
		) {
			name
			description
			maintainer
			pgpKey
			buildDate
			address
		}
	}
`;

const UPDATE_APP = gql`
	mutation Update(
		$targetName: String!
		$name: String!
		$description: String!
		$maintainer: String!
		$pgpKey: String!
		$buildDate: String!
		$address: String!
	) {
		update(
			targetName: $targetName
			name: $name
			description: $description
			maintainer: $maintainer
			pgpKey: $pgpKey
			buildDate: $buildDate
			address: $address
		) {
			name
			description
			maintainer
			pgpKey
			buildDate
			address
		}
	}
`;

const REMOVE_APP = gql`
	mutation remove($name: String!) {
		remove(name: $name) {
			name
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

const UPLOAD_MUTATION = gql`
	mutation Upload($file: Upload!) {
		singleUpload(file: $file) {
			id
			filename
			mimetype
		}
	}
`;

let s;

export default function Dashboard() {
	const [remove_app_state, setRemoveAppState] = useState(remove_template);
	const [completed, execCmds] = useState([]);
	const [new_app_state, setNewAppState] = useState(app_template);
	const [update_app_state, setUpdateAppState] = useState(update_template);
	const [current_cmd, run_cmd] = useState(cmd_template);
	const [extra_ui, setExtraUI] = useState(extra_ui_template);
	const [file_state, setFile] = useState({});
	const [upload] = useMutation(UPLOAD_MUTATION, {
		onCompleted: (res) => {
			if (res) {
				let new_cmds = completed;
				new_cmds.push({
					...current_cmd,
					res: `Succeed: ${JSON.stringify(res)}`,
				});
				execCmds([...new_cmds]);
				run_cmd(cmd_template);
				setRemoveAppState(remove_template);
			}
		},
		onError: (error) => {
			if (error) {
				let new_cmds = completed;
				new_cmds.push({
					...current_cmd,
					error: true,
					res: JSON.stringify(error),
				});
				execCmds([...new_cmds]);
				run_cmd(cmd_template);
				setExtraUI({ show: false, target: "" });
			}
		},
	});

	// Graphql
	const [create_app, c_data] = useMutation(CREATE_APP, {
		variables: {
			...new_app_state,
		},
	});

	const [updateMutation] = useMutation(UPDATE_APP, {
		variables: {
			...update_app_state,
		},
		onCompleted: (data) => {
			let { __typename, ...others } = data.update;
			setUpdateAppState({ ...others });
			setExtraUI({ show: true, target: "update" });
		},
	});
	const [removeMutation, MutationResult] = useMutation(REMOVE_APP, {
		variables: { ...remove_app_state },
		onCompleted: (res) => {
			if (res) {
				let new_cmds = completed;
				new_cmds.push({
					...current_cmd,
					res: `Succeed: ${JSON.stringify(res.remove)}`,
				});
				execCmds([...new_cmds]);
				run_cmd(cmd_template);
				setRemoveAppState(remove_template);
			}
		},
	});
	const [loadQuery] = useLazyQuery(GET_APP, {
		variables: {
			name: update_app_state.name,
		},
		onCompleted: (data) => {
			let { __typename, ...others } = data.appByName;
			setUpdateAppState({ ...others, targetName: others.name });
			setExtraUI({ show: true, target: "update" });
		},
		onError: (error) => {
			if (error) {
				let new_cmds = completed;
				new_cmds.push({
					...current_cmd,
					error: true,
					res: JSON.stringify(error),
				});
				execCmds([...new_cmds]);
				run_cmd(cmd_template);
				setExtraUI({ show: false, target: "" });
			}
		},
	});

	const mutationHandler = ({ target, appName }) => {
		switch (target) {
			case "remove":
				setRemoveAppState({ ...remove_app_state, name: appName });
				setTimeout(() => {
					removeMutation()
						.then((res) => {
							if (res) {
								console.log(res);
							}
						})
						.catch((err) => {
							if (err) {
								let new_cmds = completed;
								new_cmds.push({
									...current_cmd,
									error: true,
									res: JSON.stringify(err),
								});
								execCmds([...new_cmds]);
								run_cmd(cmd_template);
							}
						});
				}, 500);
				break;
			case "add":
				setNewAppState({ ...new_app_state, name: appName });
				setExtraUI({ show: true, target: "add" });
				break;
			case "update":
				setUpdateAppState({ ...update_app_state, name: appName });
				loadQuery();

				break;
			default:
				let new_cmds = completed;
				new_cmds.push({
					...current_cmd,
					res: "Command not found.	",
				});
				execCmds([...new_cmds]);
				run_cmd(cmd_template);

				break;
		}
	};

	useEffect(() => {}, [file_state]);
	useEffect(() => {}, [remove_app_state]);
	return (
		<div
			style={{
				width: "100%",
				display: "grid",
				gridTemplateColumns: "200px 1fr",
				padding: "0.5rem 1rem",
				lineHeight: "1.6rem",
				color: "green",
				overflow: "hidden",
				textOverflow: "ellipsis",
				wordWrap: "break-word",
			}}
		>
			{!window.sessionStorage.getItem("token") && <Redirect to="/login" />}
			{completed.length > 0 &&
				completed.map((cmd) => {
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
				onKeyDown={async (e) => {
					if (e.key === "Enter") {
						let base = current_cmd.cmd.split(" ")[0];
						if (base === "pix") {
							let verb = current_cmd.cmd.split(" ")[1];
							let app = current_cmd.cmd.split(" ")[2];
							mutationHandler({ target: verb, appName: app });
						} else {
							mutationHandler({ target: "", appName: "" });
						}
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
				<label>File Address</label>
				<input
					style={{ width: "100%" }}
					value={
						extra_ui.target === "add" ? new_app_state.address : update_app_state.address
					}
					onChange={(e) =>
						extra_ui.target === "add"
							? setNewAppState({ ...new_app_state, address: e.target.value })
							: setUpdateAppState({ ...update_app_state, address: e.target.value })
					}
				/>
				<input
					type="file"
					id="actual-btn"
					name="filename"
					hidden
					onChange={({
						target: {
							validity,
							files: [file],
						},
					}) =>
						validity.valid &&
						upload({ variables: { file } }).then((res) => {
							setNewAppState({
								...new_app_state,
								address: `http://localhost:4000/public/applications/${res.data.singleUpload.filename}`,
							});
						})
					}
				/>

				<label
					id="upload_btn"
					htmlFor="actual-btn"
					style={{ position: "absolute", left: 10, bottom: 10, padding: "0px 10px" }}
				>
					Upload
				</label>
				<button
					id="submit_btn"
					style={{ position: "absolute", right: 10, bottom: 10 }}
					onClick={(e) => {
						switch (extra_ui.target) {
							case "add":
								create_app()
									.then((res) => {
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
									})
									.catch((err) => {
										if (err) {
											let new_cmds = completed;
											new_cmds.push({
												...current_cmd,
												error: true,
												res: JSON.stringify(err),
											});
											execCmds([...new_cmds]);
											run_cmd(cmd_template);
											setExtraUI({ show: false, target: "" });
										}
									});
								break;
							case "update":
								updateMutation()
									.then(({ data, error }) => {
										if (data.update) {
											setExtraUI({ show: false, target: "" });

											let new_cmds = completed;
											new_cmds.push({
												...current_cmd,
												res: `Succeed: ${JSON.stringify(data.update)}`,
											});
											execCmds([...new_cmds]);
											run_cmd(cmd_template);
										}
									})
									.catch((err) => {
										if (err) {
											let new_cmds = completed;
											new_cmds.push({
												...current_cmd,
												error: true,
												res: JSON.stringify(err),
											});
											execCmds([...new_cmds]);
											run_cmd(cmd_template);
											setExtraUI({ show: false, target: "" });
										}
									});
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
		<div
			style={{
				width: "100%",
				gridColumn: "1/3",
				display: "grid",
				gridTemplateColumns: "200px 1fr",
			}}
		>
			<TerminalHeader style={{ gridColumn: "1/2" }} />{" "}
			<span style={{ margin: 0, padding: 0, gridColumn: "2/3" }}>{cmd.cmd}</span>
			<p style={{ width: "100%", gridColumn: "1/3", color: cmd.error ? "crimson" : "green" }}>
				{cmd.res}
			</p>
			<br />
		</div>
	);
}

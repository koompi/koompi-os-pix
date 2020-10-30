import React from "react";
import TerminalHeader from "./TermHead";
import { gql } from "@apollo/client";
import { useQuery } from "@apollo/react-hooks";
import { v4 as Uuidv4 } from "uuid";

const GET_APPS = gql`
	query {
		apps {
			name
			description
		}
	}
`;

export default function Packages() {
  const { loading, error, data } = useQuery(GET_APPS);

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
  console.log(data);
  return (
    <div style={{ padding: "0.5rem 1rem", lineHeight: "1.6rem" }}>
      <div className="css-typing">
        <TerminalHeader />
        <p
          style={{
            width: `${command.length - 1}em`,
            WebkitAnimation: `type ${command.length / 7}s steps(${command.length}, end)`,
            animation: `type ${command.length / 5}s steps(${command.length}, end)`,
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
          animationDuration: `${(command.length / 5) * 1000}ms`,
        }}
      >
        <p>Showing all packages... </p>
        <br />
        <input
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
        />
        <br />
        <br />
        <table
          style={{
            width: "100%",
            textAlign: "left"
          }}
        >
          <thead>
            <tr>
              <th>No</th>
              <th>Name</th>
              <th>Install</th>
              <th>Actions</th>
            </tr>
          </thead>
          <tbody>
            {
              data.apps.map((app, i) => (
                <tr key={Uuidv4()}>
                  <td>{i + 1}</td>
                  <td>{app.name}</td>
                  <td>pix install {app.name}</td>
                  <td>
                    <button style={action_btn}>Copy</button>
                    <button style={action_btn}>Info</button>
                  </td>
                </tr>
              ))
            }
          </tbody>
        </table>
      </div>
    </div>
  );
}

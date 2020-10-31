import React from 'react'
import TerminalHeader from "./TermHead"

export default function Login() {
    let command = "pix --login";
    return (
        <div style={{ padding: "0.5rem 1rem", lineHeight: "1.6rem" }}>
            <div className="css-typing">
                <TerminalHeader /><p style={{
                    width: `${command.length - 2}em`,
                    "WebkitAnimation": `type ${command.length / 5}s steps(${command.length}, end)`,
                    animation: `type ${command.length / 5}s steps(${command.length}, end)`,
                    "WebkitAnimationFillMode": "forwards",
                    "AnimationFillMode": "forwards"
                }}>{command}</p>

            </div>
            <div style={{animationName: "show", "animationDuration": `${command.length / 5 * 1000}ms`, display: "grid", gridTemplateColumns: "1fr", gridGap: "5px" }}>
                <br/>
                <label>Email: </label>
                <input type="email" style={{width: "100%", border: "2px solid green", color: "green"}}/>
                <br/>
                <br/>
                <label>Password: </label>
                <input type="password" style={{width: "100%", border: "2px solid green", color: "green"}}/>
                <br/>
                <br/>
                <button style={{width: "10%", border: "2px solid green", color: "green", justifySelf: "end"}}>Login</button>
            </div>
        </div>
    )
}

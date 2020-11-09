import React from "react";
import TerminalHeader from "./TermHead";

export default function NotFound() {
	return (
		<div>
			<div className="css-typing">
				<TerminalHeader />
				<p
					style={{
						width: `${window.location.pathname.length - 1}em`,
						WebkitAnimation: `type ${window.location.pathname.length / 8}s steps(${
							window.location.pathname.length + 1
						}, end)`,
						animation: `type ${window.location.pathname.length / 8}s steps(${
							window.location.pathname.length + 1
						}, end)`,
						WebkitAnimationFillMode: "forwards",
						AnimationFillMode: "forwards",
					}}
				>
					cd {window.location.pathname}
				</p>
			</div>
			<p
				style={{
					color: "crimson ",
					animationName: "show",
					animationDuration: `${(window.location.pathname.length / 8) * 1000}ms`,
				}}
			>
				404 Path not found!
			</p>
		</div>
	);
}

import TerminalHeader from "./TermHead";

const welcome = "pix --help";
export default function Home() {
	return (
		<div className="home_header">
			<div className="css-typing">
				<TerminalHeader />
				<p className="command">{welcome}</p>
			</div>
			<div className="entered">
				<br />
				<p>WELCOME!</p>
				<br />
				<p>
					<span className="pix">pix</span> helps automate and simply the complex
					installations and configurations to be done in just one command. It created by{" "}
					<a href="https://koompi.org">KOOMPI OS</a> in order that opensource software
					become mass adoption friendly.
				</p>
				<br />
				<p>INSTALLATION</p>
				<br />
				<p>
					Copy and paste the command below into your terminal then press enter. Please
					make sure you have admin privilege.
				</p>
				<div className="box">
					<br />
					curl -Ssf https://pix.koompi.org/installer.sh | sh
					<button className="cp_btn">copy</button>
					<br />
					<br />
				</div>
			</div>
		</div>
	);
}

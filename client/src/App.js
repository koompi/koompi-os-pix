import { Switch, Route, NavLink, Redirect } from "react-router-dom";
import NotFound from "./NotFound";
import Home from "./Home";
import Packages from "./Packages";
import Login from "./Login";
import Dashboard from "./Dashboard";

function App() {
	return (
		<div className="AppContainer">
			{ window.location.protocol !== "https:" && <Redirect to={`https://${window.location.host}${window.location.pathname}`} />}
			<div className="tab_bar">
				<NavLink exact className="nav" to="/" activeClassName="active">
					Home
				</NavLink>
				<NavLink exact className="nav" to="/packages" activeClassName="active">
					Packages
				</NavLink>
			</div>
			<div className="pageContainer">
				<Switch>
					<Route exact={true} path="/" component={Home} />
					<Route exact={true} path="/login" component={Login} />
					<Route exact={true} path="/packages" component={Packages} />
					<Route exact={true} path="/dashboard" component={Dashboard} />
					<Route exact={true} component={NotFound} />
				</Switch>
			</div>
		</div>
	);
}

export default App;

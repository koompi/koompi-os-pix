import { Switch, Route, Redirect, NavLink } from 'react-router-dom';

import NotFound from './NotFound';
import Home from './Home';
import About from './About';
import Packages from './Packages';

function App() {
  console.log(window.location.pathname === "/")
  return (
    <div className="AppContainer">
      <div className="tab_bar">
        {/* <div className={window.location.pathname === "/" ? "active" : ""}>Home</div>
        <div className={window.location.pathname === "/packages" ? "active" : ""}>Packages</div> */}
        <NavLink exact className="nav" to="/" activeClassName="active">
          Home
        </NavLink>
        <NavLink exact className="nav" to="/packages" activeClassName="active">
          Packages
        </NavLink>
      </div>
      <div>
        <Switch className="App">
          <Route exact={true} path="/" component={Home} />
          <Route exact={true} path="/about" component={About} />
          <Route exact={true} path="/packages" component={Packages} />
          <Route exact={true} component={NotFound} />
        </Switch>
      </div>

    </div>
  );
}

export default App;

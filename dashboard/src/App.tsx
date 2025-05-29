import { Navigate, Route, BrowserRouter as Router, Routes } from "react-router-dom"
import './global.css'
import { useGlobal } from "./context/GlobalContext"
import { Home } from "./pages/Home"
import { useSetup } from "./context/SetupContext";
import Loading from "./components/Loading";
import { Setup } from "./pages/Setup";
import { Login } from "./pages/Login";
import { useAuth } from "./context/authContext";
import { ToastContainer } from "react-toastify";
import { Layout } from "./Layout";

function App() {
  
  const { needsSetup } = useSetup();
  const { globalLoading: loading } = useGlobal();
  const { user } = useAuth();
  if (loading) {
    return <Loading />
  }

  return (
    <Router>
      <Routes>
      {needsSetup && (
          <>
            <Route path="/setup" element={<Setup />} />
            <Route path="*" element={<Navigate to="/setup" />} />
          </>
        )}
        {!needsSetup && !user && (
          <>
            <Route path="/login" element={<Login />} />
            <Route path="*" element={<Navigate to="/login" />} />
          </>
        )}
        {!needsSetup && user && (
          <Route element={<Layout />}>
            <Route path="/" element={<Home />} />
            <Route path="/projects" element={<div>Projects Page</div>} />
            <Route path="/databases" element={<div>Databases Page</div>} />
            <Route path="/settings" element={<div>Settings Page</div>} />
            <Route path="/deployments" element={<div>Deployments Page</div>} />
            <Route path="/logs" element={<div>Logs Page</div>} />
            {/* <Route path="/terminal" element={<TerminalPage />} /> */}
            <Route path="/users" element={<div>Users Page</div>} />
            <Route path="/settings" element={<div>Settings Page</div>} />
            <Route path="/login" element={<Navigate to="/" />} />
            <Route path="*" element={<Navigate to="/" />} />
          </Route>
        )}
      </Routes>
      <ToastContainer />
    </Router>
  )
}

export default App



// "background": "#0D1117",
//     "foreground": "#C9D1D9",
//     "primary": "#1F6FEB",
//     "accent": "#A371F7",
//     "success": "#2EA043",
//     "warning": "#FBBF24",
//     "error": "#F85149",
//     "card": "#161B22",
//     "border": "#30363D"
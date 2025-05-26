
import Loading from "../components/Loading";
import { useAuth } from "../context/authContext";
import { useGlobal } from "../context/GlobalContext";



export const Home = () => {
    const { globalLoading } = useGlobal();
    const { user } = useAuth();

    if (globalLoading) {
        return <Loading />
    }

    return (
        <p className="text-white text-2xl">
            {user ? `Welcome, ${user.username}! you are ${user.isAdmin ? "admin" : "not an admin"}` : "Welcome to the Dashboard!"}
        </p>
    )
}
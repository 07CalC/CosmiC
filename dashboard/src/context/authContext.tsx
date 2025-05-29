import React, { createContext, useContext, useEffect, useState } from "react";
import { useGlobal } from "./GlobalContext";
import { toast } from "react-toastify";

interface User {
  id: string;
  username: string;
  email: string;
  isAdmin: boolean;
  role: "owner" | "admin" | "user";
}

interface AuthContextType {
  user: User | null;
  setUser: React.Dispatch<React.SetStateAction<User | null>>;
  logout: () => void;
  loading: boolean;
}

const AuthContext = createContext<AuthContextType>({
  user: null,
  setUser: () => {},
  logout: () => {},
  loading: true,
});

export const AuthProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);
  const { setGlobalLoading } = useGlobal();

  const fetchUser = async () => {
    try {
      setGlobalLoading(true);
      const res = await fetch("/api/auth/me", {
        method: "GET",
        headers: {
          "Content-Type": "application/json",
        },
        credentials: "include",
      });

      const data = await res.json();
      if(!data.success){
        toast.error(data.error || "Failed to fetch user data");
        setUser(null);
        return;
      }
      setUser(data.user);
    } catch (err) {
      setUser(null);
    } finally {
      setLoading(false);
      setGlobalLoading(false);
    }
  };

  useEffect(() => {
    fetchUser();
  }, []);


  const logout = async () => {
    const res = await fetch("/api/auth/logout", {
      method: "GET",
      headers: {
        "Content-Type": "application/json",
      },
      credentials: "include",
    });
    const data = await res.json();
    if(data.success){
      toast.success("Logged out successfully");
      setUser(null);
    }
  };

  return (
    <AuthContext.Provider value={{ user, setUser, logout, loading }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => useContext(AuthContext);

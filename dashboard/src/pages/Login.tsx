import { useState } from 'react';
import { useAuth } from '../context/authContext';

export const Login = () => {
    const [formData, setFormData] = useState({
        username: '',
        password: ''
    });
    const [error, setError] = useState<string | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const { setUser } = useAuth();

    const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        const { id, value } = e.target;
        setFormData(prev => ({
            ...prev,
            [id]: value
        }));
    };

    const handleSubmit = async (event: React.FormEvent<HTMLFormElement>) => {
        event.preventDefault();
        setIsLoading(true);
        setError(null);

        try {
            const response = await fetch('/api/auth/login', {
                method: 'POST',
                headers: {
                    'Content-Type': 'application/json',
                },
                body: JSON.stringify(formData),
                credentials: 'include' 
            });

            const data = await response.json();
            if(!data.success){
                setError(data.error)
            }
            if (response.ok){
                setUser(data.user);
            }
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Something went wrong');
        } finally {
            setIsLoading(false);
        }
    };
    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-[#0D1117]">
            <h1 className="text-2xl font-bold text-[#C9D1D9] mb-4">Login</h1>
            {error && (
                <div className="mb-4 p-3 bg-[#F85149] bg-opacity-20 border border-[#F85149] rounded-md text-white">
                    {error}
                </div>
            )}
            <form onSubmit={handleSubmit} className="w-full max-w-sm bg-[#161B22] p-6 rounded-lg border border-[#30363D]">
                <div className="mb-4">
                    <label 
                        className="block text-[#C9D1D9] mb-2" 
                        htmlFor="username"
                    >
                        Username
                    </label>
                    <input
                        type="text"
                        id="username"
                        value={formData.username}
                        onChange={handleChange}
                        className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md 
                                 text-[#C9D1D9] placeholder-gray-500
                                 focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:border-transparent"
                        placeholder="Enter your username"
                        required
                    />
                </div>
                <div className="mb-6">
                    <label 
                        className="block text-[#C9D1D9] mb-2" 
                        htmlFor="password"
                    >
                        Password
                    </label>
                    <input
                        type="password"
                        id="password"
                        value={formData.password}
                        onChange={handleChange}
                        className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md 
                                 text-[#C9D1D9] placeholder-gray-500
                                 focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:border-transparent"
                        placeholder="Enter your password"
                        required
                    />
                </div>
                <button
                    type="submit"
                    disabled={isLoading}
                    className="w-full bg-[#1F6FEB] hover:bg-[#1958BD] text-[#C9D1D9] 
                             font-semibold py-2 px-4 rounded-md
                             focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:ring-offset-2 
                             focus:ring-offset-[#161B22] transition duration-200
                             disabled:opacity-50 disabled:cursor-not-allowed"
                >
                    {isLoading ? 'Logging in...' : 'Login'}
                </button>
            </form>
        </div>
    );
};
import React, { useState } from 'react';

export const Setup: React.FC = () => {
    const [formData, setFormData] = useState({
        email: '',
        username: '',
        password: '',
    });

    const handleSubmit = (e: React.FormEvent) => {
        e.preventDefault();
        // Handle form submission
    };

    return (
        <div className="flex flex-col items-center justify-center min-h-screen bg-[#0D1117]">
            <h1 className="text-2xl font-bold text-[#C9D1D9] mb-4">Setup Admin Account</h1>
            <form 
                onSubmit={handleSubmit}
                className="w-full max-w-sm bg-[#161B22] p-6 rounded-lg border border-[#30363D]"
            >
                <div className="mb-4">
                    <label 
                        className="block text-[#C9D1D9] mb-2" 
                        htmlFor="email"
                    >
                        Email
                    </label>
                    <input
                        type="email"
                        id="email"
                        className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md 
                                 text-[#C9D1D9] placeholder-gray-500
                                 focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:border-transparent"
                        placeholder="Enter your email"
                        required
                    />
                </div>
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
                        className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md 
                                 text-[#C9D1D9] placeholder-gray-500
                                 focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:border-transparent"
                        placeholder="Choose a username"
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
                        className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md 
                                 text-[#C9D1D9] placeholder-gray-500
                                 focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:border-transparent"
                        placeholder="Create a strong password"
                        required
                    />
                </div>
                <button
                    type="submit"
                    className="w-full bg-[#1F6FEB] hover:bg-[#1958BD] text-[#C9D1D9] 
                             font-semibold py-2 px-4 rounded-md
                             focus:outline-none focus:ring-2 focus:ring-[#1F6FEB] focus:ring-offset-2 
                             focus:ring-offset-[#161B22] transition duration-200"
                >
                    Complete Setup
                </button>
            </form>
            <p className="mt-4 text-sm text-[#C9D1D9]">
                This will create your admin account
            </p>
        </div>
    );
};
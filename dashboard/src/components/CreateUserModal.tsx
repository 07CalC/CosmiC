import { useState } from 'react';
import { useAuth } from '../context/authContext';

interface CreateUserModalProps {
    isOpen: boolean;
    onClose: () => void;
    onSubmit: (userData: { username: string; email: string; password: string; is_admin: boolean }) => void;
}

export const CreateUserModal = ({ isOpen, onClose, onSubmit }: CreateUserModalProps) => {
    const [formData, setFormData] = useState({
        username: '',
        email: '',
        password: '',
        is_admin: false
    });
    const { user } = useAuth();

    if (!isOpen) return null;

    return (
        <div className="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
            <div className="bg-[#161B22] border border-[#30363D] rounded-lg p-6 w-full max-w-md">
                <h2 className="text-[#C9D1D9] text-xl font-semibold mb-4">Create New User</h2>
                
                <form onSubmit={(e) => {
                    e.preventDefault();
                    onSubmit(formData);
                    setFormData({ username: '', email: '', password: '', is_admin: false });
                }}>
                    <div className="space-y-4">
                        <div>
                            <label className="block text-[#8B949E] text-sm mb-2">Username</label>
                            <input
                                type="text"
                                className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md text-[#C9D1D9] focus:outline-none focus:border-[#1F6FEB]"
                                value={formData.username}
                                onChange={(e) => setFormData(prev => ({ ...prev, username: e.target.value }))}
                                required
                            />
                        </div>
                        
                        <div>
                            <label className="block text-[#8B949E] text-sm mb-2">Email</label>
                            <input
                                type="email"
                                className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md text-[#C9D1D9] focus:outline-none focus:border-[#1F6FEB]"
                                value={formData.email}
                                onChange={(e) => setFormData(prev => ({ ...prev, email: e.target.value }))}
                                required
                            />
                        </div>
                        
                        <div>
                            <label className="block text-[#8B949E] text-sm mb-2">Password</label>
                            <input
                                type="password"
                                className="w-full px-3 py-2 bg-[#0D1117] border border-[#30363D] rounded-md text-[#C9D1D9] focus:outline-none focus:border-[#1F6FEB]"
                                value={formData.password}
                                onChange={(e) => setFormData(prev => ({ ...prev, password: e.target.value }))}
                                required
                            />
                        </div>
                        
                        <div className="flex items-center">
                            <input
                                type="checkbox"
                                id="is_admin"
                                className="mr-2"
                                checked={formData.is_admin}
                                disabled={user?.role !== 'owner'}
                                onChange={(e) => setFormData(prev => ({ ...prev, is_admin: e.target.checked }))}
                            />
                            <label htmlFor="is_admin" className="text-[#8B949E] text-sm">Make user an admin</label>
                        </div>
                    </div>
                    
                    <div className="flex justify-end gap-3 mt-6">
                        <button
                            type="button"
                            onClick={onClose}
                            className="px-4 py-2 text-[#C9D1D9] hover:bg-[#21262D] rounded-md transition-colors"
                        >
                            Cancel
                        </button>
                        <button
                            type="submit"
                            className="px-4 py-2 bg-[#1F6FEB] text-white rounded-md hover:bg-[#1A73E8] transition-colors"
                        >
                            Create User
                        </button>
                    </div>
                </form>
            </div>
        </div>
    );
};
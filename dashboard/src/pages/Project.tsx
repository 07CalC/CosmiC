import { useEffect, useState } from "react";
import { useParams, useNavigate } from "react-router-dom";
import { toast } from "react-toastify";
import Loading from "../components/Loading";
import { useAuth } from "../context/authContext";
import { FiEdit2, FiTrash2 } from 'react-icons/fi';
import { EditProjectModal } from '../components/EditProjectModal';

interface User {
    id: string;
    username: string;
    email: string;
    is_admin: boolean;
    role: 'owner' | 'admin' | 'user';
}

interface Project {
    id: string;
    name: string;
    description?: string;
    tags: string[];
    owner: User;
    created_at: string;
    updated_at: string;
}

interface App {
    id: string;
    name: string;
    repo_url: string;
    branch: string;
    app_type: string;
    domain?: string;
    port?: number;
    created_by: User;
    updated_at: string;
}

interface ProjectResponse {
    success: boolean;
    error: string | null;
    project: Project;
    apps: App[];
}

export const ProjectPage = () => {
    const { user } = useAuth();
    const params = useParams();
    const navigate = useNavigate();
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [project, setProject] = useState<Project | null>(null);
    const [apps, setApps] = useState<App[]>([]);
    const [isEditModalOpen, setIsEditModalOpen] = useState(false);

    const fetchProjectDetails = async () => {
        try {
            const response = await fetch(`/api/projects/${params.projectId}`, {
                method: 'GET',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include'
            });
            const data: ProjectResponse = await response.json();
            
            if (!data.success) {
                toast.error(data.error || 'Failed to fetch project details');
                throw new Error(data.error || 'Failed to fetch project details');
            }

            setProject(data.project);
            setApps(data.apps);
        } catch (err) {
            setError(err instanceof Error ? err.message : 'Failed to fetch project details');
            toast.error('Failed to fetch project details');
        } finally {
            setLoading(false);
        }
    };

    const handleDelete = async () => {
        if (!window.confirm('Are you sure you want to delete this project? This action cannot be undone.')) {
            return;
        }

        try {
            const response = await fetch(`/api/projects/${project?.id}`, {
                method: 'DELETE',
                credentials: 'include'
            });
            console.log(response);
            const data = await response.json();
            
            if (!data.success) {
                toast.error(data.error || 'Failed to delete project');
                throw new Error(data.error || 'Failed to delete project');
            }

            toast.success('Project deleted successfully');
            navigate('/projects');
        } catch (err) {
            toast.error(err instanceof Error ? err.message : 'Failed to delete project');
        }
    };

    const handleEdit = async (projectData: { name: string; description: string; tags: string[] }) => {
        try {
            const response = await fetch(`/api/projects/${project?.id}`, {
                method: 'PUT',
                headers: {
                    'Content-Type': 'application/json',
                },
                credentials: 'include',
                body: JSON.stringify(projectData),
            });

            const data = await response.json();

            if (!data.success) {
                toast.error(data.error || 'Failed to update project');
                throw new Error(data.error || 'Failed to update project');
            }

            toast.success(data.message || 'Project updated successfully');
            fetchProjectDetails();
            setIsEditModalOpen(false);
        } catch (err) {
            toast.error(err instanceof Error ? err.message : 'Failed to update project');
        }
    };

    useEffect(() => {
        fetchProjectDetails();
    }, [params.projectId]);

    if (loading) return <Loading />;

    if (error) {
        return (
            <div className="min-h-screen bg-[#0D1117] p-6">
                <div className="bg-[#F8514933] border border-[#F85149] text-[#F85149] p-4 rounded-lg">
                    {error}
                </div>
            </div>
        );
    }

    if (!project) return null;

    return (
        <div className="min-h-screen bg-[#0D1117] p-6">
            <div className="flex justify-between items-start mb-6">
                <div>
                    <h1 className="text-[#C9D1D9] text-2xl font-bold">{project.name}</h1>
                    <p className="text-[#8B949E] mt-1">{project.description}</p>
                    <div className="flex flex-wrap gap-2 mt-3">
                        {project.tags.map((tag) => (
                            <span key={tag} className="px-2 py-1 text-xs rounded-full bg-[#1F6FEB33] text-[#1F6FEB]">
                                {tag}
                            </span>
                        ))}
                    </div>
                    <div className="flex items-center gap-4 mt-4 text-[#8B949E] text-sm">
                        <div className="flex items-center gap-2">
                            <div className="w-5 h-5 rounded-full bg-[#30363D] flex items-center justify-center">
                                <span className="text-[#C9D1D9] text-xs">
                                    {project.owner.username[0].toUpperCase()}
                                </span>
                            </div>
                            <span>Created by {project.owner.username}</span>
                        </div>
                        <span>·</span>
                        <span>Updated {new Date(project.updated_at).toLocaleDateString()}</span>
                    </div>
                </div>
                {user?.isAdmin && (
                    <div className="flex items-center gap-2">
                        <button 
                            onClick={() => setIsEditModalOpen(true)}
                            className="px-3 py-2 bg-[#21262D] text-[#C9D1D9] rounded-lg hover:bg-[#30363D] transition-colors inline-flex items-center gap-2"
                        >
                            <FiEdit2 className="w-4 h-4" />
                            Edit
                        </button>
                        <button 
                            onClick={() => handleDelete()}
                            className="px-3 py-2 bg-[#21262D] text-[#F85149] rounded-lg hover:bg-[#30363D] transition-colors inline-flex items-center gap-2"
                        >
                            <FiTrash2 className="w-4 h-4" />
                            Delete
                        </button>
                        <button className="px-3 py-2 bg-[#1F6FEB] text-white rounded-lg hover:bg-[#1A73E8] transition-colors inline-flex items-center gap-2">
                            <svg className="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                            </svg>
                            New App
                        </button>
                    </div>
                )}
            </div>

            <div className="mt-8">
                <h2 className="text-[#C9D1D9] text-xl font-semibold mb-4">Applications</h2>
                {apps.length === 0 ? (
                    <div className="bg-[#161B22] border border-[#30363D] rounded-lg p-8 text-center">
                        <p className="text-[#C9D1D9] text-lg mb-4">No applications yet</p>
                        {user?.isAdmin && (
                            <button className="px-4 py-2 bg-[#1F6FEB] text-white rounded-lg hover:bg-[#1A73E8] transition-colors inline-flex items-center gap-2">
                                <svg className="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 4v16m8-8H4" />
                                </svg>
                                Deploy your first app
                            </button>
                        )}
                    </div>
                ) : (
                    <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
                        {apps.map((app) => (
                            <div key={app.id} className="bg-[#161B22] border border-[#30363D] rounded-lg p-4 hover:border-[#1F6FEB] transition-colors">
                                <div className="flex items-start justify-between">
                                    <div className="flex-1">
                                        <h3 className="text-[#C9D1D9] font-semibold text-lg">{app.name}</h3>
                                        <p className="text-[#8B949E] text-sm mt-1">{app.repo_url}</p>
                                    </div>
                                    <span className="px-2 py-1 text-xs rounded-full bg-[#1F6FEB33] text-[#1F6FEB]">
                                        {app.app_type}
                                    </span>
                                </div>
                                
                                <div className="mt-4 pt-4 border-t border-[#30363D]">
                                    <div className="flex items-center justify-between">
                                        <div className="flex items-center gap-2">
                                            <div className="w-6 h-6 rounded-full bg-[#30363D] flex items-center justify-center">
                                                <span className="text-[#C9D1D9] text-xs">
                                                    {app.created_by.username[0].toUpperCase()}
                                                </span>
                                            </div>
                                            <span className="text-[#8B949E] text-sm">
                                                {app.created_by.username}
                                            </span>
                                        </div>
                                        <span className="text-[#8B949E] text-sm">
                                            {new Date(app.updated_at).toLocaleDateString()}
                                        </span>
                                    </div>
                                </div>
                            </div>
                        ))}
                    </div>
                )}
            </div>

            {project && (
                <EditProjectModal
                    isOpen={isEditModalOpen}
                    onClose={() => setIsEditModalOpen(false)}
                    onSubmit={handleEdit}
                    project={project}
                />
            )}
        </div>
    );
};
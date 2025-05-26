import { useEffect, useState, useRef } from "react";
import Loading from "../components/Loading";
import { useGlobal } from "../context/GlobalContext";
import { XAxis, YAxis, CartesianGrid, Tooltip, ResponsiveContainer, Area, AreaChart, BarChart, Bar } from 'recharts';

interface SystemStats {
  cpuUsage: number;
  memory: {
    total: number;
    used: number;
  };
  timestamp: number;
}

export const Home = () => {
    const { globalLoading } = useGlobal();
    const [loading, setLoading] = useState(true);
    const wsRef = useRef<WebSocket | null>(null);
    const [stats, setStats] = useState<SystemStats[]>([]);

    useEffect(() => {
        if (!wsRef.current) {
            wsRef.current = new WebSocket("/api/ws/stats");

            wsRef.current.onopen = () => {
                setLoading(false);
            };

            wsRef.current.onmessage = (event) => {
                const data: SystemStats = JSON.parse(event.data);
                setStats(prev => [...prev.slice(-30), data]); // Keep last 30 data points
            };

            wsRef.current.onerror = (error) => {
                console.error("WebSocket error:", error);
                setLoading(false);
            };
        }

        return () => {
            if (wsRef.current) {
                console.log("Closing WebSocket connection");
                wsRef.current.close();
                wsRef.current = null;
            }
        };
    }, []);

    const formatMemory = (bytes: number) => {
        const gb = bytes / (1024 * 1024 * 1024);
        return `${gb.toFixed(2)} GB`;
    };


    const customTooltip = ({ active, payload, label }: any) => {
        if (active && payload && payload.length) {
            return (
                <div className="bg-[#161B22] p-3 border border-[#30363D] rounded-md">
                    <p className="text-[#C9D1D9]">
                        {new Date(label * 1000).toLocaleTimeString()}
                    </p>
                    {payload.map((pld: any, index: number) => (
                        <p key={index} style={{ color: pld.color }}>
                            {pld.dataKey === 'cpuUsage' 
                                ? `CPU: ${pld.value.toFixed(1)}%`
                                : `Memory: ${formatMemory(pld.value)} / ${formatMemory(stats[stats.length - 1]?.memory.total)}`
                            }
                        </p>
                    ))}
                </div>
            );
        }
        return null;
    };

    if (globalLoading || loading) {
        return <Loading />
    }

    return (
        <div className="p-6 space-y-6">
            <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
                {/* CPU Usage Card */}
                <div className="bg-[#161B22] p-6 rounded-lg border border-[#30363D]">
                    <h2 className="text-[#C9D1D9] text-xl font-semibold mb-4">CPU Usage</h2>
                    <div className="h-[300px]">
                        <ResponsiveContainer width="100%" height="100%">
                            <AreaChart data={stats}>
                                <CartesianGrid strokeDasharray="3 3" stroke="#30363D" />
                                <XAxis 
                                    dataKey="timestamp" 
                                    stroke="#C9D1D9"
                                    tickFormatter={(timestamp) => new Date(timestamp * 1000).toLocaleTimeString()}
                                />
                                <YAxis 
                                    stroke="#C9D1D9" 
                                    domain={[0, 100]}
                                    tickFormatter={(value) => `${value}%`}
                                />
                                <Tooltip content={customTooltip} />
                                <Area 
                                    type="monotone" 
                                    dataKey="cpuUsage" 
                                    stroke="#1F6FEB" 
                                    fill="#1F6FEB33"
                                    strokeWidth={2}
                                />
                            </AreaChart>
                        </ResponsiveContainer>
                    </div>
                    {/* CPU Stats */}
                    {stats.length > 0 && (
                        <div className="mt-4 grid grid-cols-2 gap-4">
                            <div className="bg-[#0D1117] p-4 rounded-lg border border-[#30363D]">
                                <p className="text-[#8B949E] text-sm">Current Usage</p>
                                <p className="text-[#C9D1D9] text-lg font-semibold">
                                    {stats[stats.length - 1].cpuUsage.toFixed(1)}%
                                </p>
                            </div>
                            <div className="bg-[#0D1117] p-4 rounded-lg border border-[#30363D]">
                                <p className="text-[#8B949E] text-sm">Average Usage</p>
                                <p className="text-[#C9D1D9] text-lg font-semibold">
                                    {(stats.reduce((acc, curr) => acc + curr.cpuUsage, 0) / stats.length).toFixed(1)}%
                                </p>
                            </div>
                        </div>
                    )}
                </div>

                {/* Memory Usage Card */}
                <div className="bg-[#161B22] p-6 rounded-lg border border-[#30363D]">
                    <h2 className="text-[#C9D1D9] text-xl font-semibold mb-4">Memory Usage</h2>
                    <div className="h-[300px]">
                        <ResponsiveContainer width="100%" height="100%">
                            <BarChart data={stats}>
                                <CartesianGrid strokeDasharray="3 3" stroke="#30363D" />
                                <XAxis 
                                    dataKey="timestamp" 
                                    stroke="#C9D1D9"
                                    tickFormatter={(timestamp) => new Date(timestamp * 1000).toLocaleTimeString()}
                                />
                                <YAxis 
                                    stroke="#C9D1D9"
                                    domain={[0, stats[0]?.memory.total || 'dataMax']}
                                    tickFormatter={(value) => `${formatMemory(value)}`}
                                />
                                <Tooltip content={customTooltip} />
                                <Bar 
                                    dataKey="memory.used"
                                    fill="#A371F7"
                                    radius={[4, 4, 0, 0]}
                                />
                            </BarChart>
                        </ResponsiveContainer>
                    </div>
                    {/* Memory Stats */}
                    {stats.length > 0 && (
                        <div className="mt-4 grid grid-cols-2 gap-4">
                            <div className="bg-[#0D1117] p-4 rounded-lg border border-[#30363D]">
                                <p className="text-[#8B949E] text-sm">Used Memory</p>
                                <p className="text-[#C9D1D9] text-lg font-semibold">
                                    {formatMemory(stats[stats.length - 1].memory.used)}
                                </p>
                            </div>
                            <div className="bg-[#0D1117] p-4 rounded-lg border border-[#30363D]">
                                <p className="text-[#8B949E] text-sm">Total Memory</p>
                                <p className="text-[#C9D1D9] text-lg font-semibold">
                                    {formatMemory(stats[stats.length - 1].memory.total)}
                                </p>
                            </div>
                        </div>
                    )}
                </div>
            </div>
        </div>
    );
}
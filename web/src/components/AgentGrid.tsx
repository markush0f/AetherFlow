import React, { useState, useEffect } from 'react';
import { LucideAlertCircle, LucideComponent, LucideSettings } from './icons';

export default function AgentGrid() {
    const [agents, setAgents] = useState<any[]>([]);
    const [status, setStatus] = useState('Connecting to Gateway...');
    const [statusColor, setStatusColor] = useState('text-yellow-500');
    const [loading, setLoading] = useState(true);

    useEffect(() => {
        let ws: WebSocket | null = null;
        let reconnectTimeout: any;

        const connect = () => {
            setStatus('Connecting to Gateway...');
            setStatusColor('text-yellow-500');

            ws = new WebSocket("ws://localhost:8080/ws");

            ws.onopen = () => {
                setStatus('Connected to AetherFlow WebSocket Endpoint');
                setStatusColor('text-green-500');
            };

            ws.onmessage = (event) => {
                try {
                    const parsedAgents = JSON.parse(event.data);
                    parsedAgents.sort((a: any, b: any) => a.slug.localeCompare(b.slug));
                    setAgents(parsedAgents);
                    setLoading(false);
                } catch (e) {
                    console.error("Failed to parse agents JSON", e);
                }
            };

            ws.onclose = () => {
                setStatus('Connection lost. Reconnecting...');
                setStatusColor('text-red-500');
                reconnectTimeout = setTimeout(connect, 3000);
            };

            ws.onerror = (err) => {
                console.error("WebSocket error:", err);
            };
        };

        connect();

        return () => {
            if (reconnectTimeout) clearTimeout(reconnectTimeout);
            if (ws) {
                ws.onclose = null;
                ws.close();
            }
        };
    }, []);

    const renderStatusBadge = (status: string) => {
        if (status === "Ready") {
            return (
                <span className="inline-flex items-center px-2.5 py-1 rounded-md text-xs font-semibold bg-emerald-500/10 text-emerald-400 border border-emerald-500/20">
                    <span className="w-1.5 h-1.5 rounded-full bg-emerald-500 mr-2 animate-pulse"></span>
                    Ready
                </span>
            );
        } else if (status === "Pending") {
            return (
                <span className="inline-flex items-center px-2.5 py-1 rounded-md text-xs font-semibold bg-amber-500/10 text-amber-500 border border-amber-500/20">
                    <span className="w-1.5 h-1.5 rounded-full bg-amber-500 mr-2"></span>
                    Pending
                </span>
            );
        } else if (status === "Unreachable") {
            return (
                <span className="inline-flex items-center px-2.5 py-1 rounded-md text-xs font-semibold bg-rose-500/10 text-rose-400 border border-rose-500/20">
                    <LucideAlertCircle className="w-3 h-3 mr-1.5" />
                    Unreachable
                </span>
            );
        } else {
            return (
                <span className="inline-flex items-center px-2.5 py-1 rounded-md text-xs font-semibold bg-gray-500/10 text-gray-400 border border-gray-500/20">{status}</span>
            );
        }
    };

    return (
        <>
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {loading ? (
                    <div className="col-span-full flex justify-center py-20 bg-[#1e2124] rounded-lg border border-gray-700/50 shadow-sm">
                        <div className="animate-pulse flex flex-col items-center">
                            <div className="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin mb-4"></div>
                            <p className="text-gray-400 text-sm">Connecting to WebSockets...</p>
                        </div>
                    </div>
                ) : agents.length === 0 ? (
                    <div className="col-span-full py-12 text-center bg-gray-800/50 rounded-2xl border border-gray-700 border-dashed">
                        <p className="text-gray-400">No agents found in the AetherFlow database.</p>
                    </div>
                ) : (
                    agents.map((agent) => (
                        <div key={agent.id} className="bg-[#111318] border border-gray-800 rounded-2xl shadow-sm hover:border-indigo-500/40 hover:shadow-[0_4px_20px_-4px_rgba(99,102,241,0.1)] transition-all duration-300 flex flex-col h-full overflow-hidden">
                            <div className="p-6 flex-grow">
                                <div className="flex justify-between items-center mb-5">
                                    <h3 className="text-xl font-bold text-gray-100 capitalize tracking-tight">{agent.slug.replace(/-/g, " ")}</h3>
                                    {renderStatusBadge(agent.status)}
                                </div>

                                <div className="space-y-4">
                                    <div className="bg-gray-800/30 p-3 rounded-xl border border-gray-800/60">
                                        <span className="text-gray-500 text-[10px] uppercase font-bold tracking-wider block mb-1">UUID</span>
                                        <span className="font-mono text-xs text-gray-300 block truncate">{agent.id}</span>
                                    </div>

                                    <div className="bg-gray-800/30 p-3 rounded-xl border border-gray-800/60">
                                        <span className="text-gray-500 text-[10px] uppercase font-bold tracking-wider block mb-1">Target</span>
                                        <span className="text-indigo-400 block text-sm truncate">{agent.endpoint}</span>
                                    </div>

                                    {agent.source && (
                                        <div className="bg-gray-800/30 p-3 rounded-xl border border-gray-800/60">
                                            <span className="text-gray-500 text-[10px] uppercase font-bold tracking-wider block mb-1">Source Component</span>
                                            <span className="text-sm text-gray-300 block truncate flex items-center gap-2">
                                                <LucideComponent className="w-3.5 h-3.5 text-gray-500" />
                                                {agent.source}
                                            </span>
                                        </div>
                                    )}
                                </div>
                            </div>

                            <div className="px-6 py-4 bg-gray-900/50 border-t border-gray-800 mt-auto flex gap-3">
                                <button className="flex-1 bg-white hover:bg-gray-200 text-gray-900 font-semibold py-2 px-4 rounded-xl text-sm transition-all shadow-sm active:scale-[0.98]" onClick={() => alert(`Execute task on ${agent.slug}`)}>
                                    Execute Action
                                </button>
                                <button className="flex-none bg-[#111318] hover:bg-gray-800 text-gray-400 hover:text-white p-2 border border-gray-800 rounded-xl transition-all" title="Settings">
                                    <LucideSettings className="w-5 h-5 mx-auto" />
                                </button>
                            </div>
                        </div>
                    ))
                )}
            </div>

            <div className={`mt-8 text-center text-sm ${statusColor}`}>
                {status}
            </div>
        </>
    );
}

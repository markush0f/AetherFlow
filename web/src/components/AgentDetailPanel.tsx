import React, { useState, useEffect } from "react";
import { LucideX, LucideGlobe, LucideTerminal, LucideCode, LucidePlus, LucideTrash, LucideServer, LucideClipboard } from "./icons";
import { CreateTaskDialog } from "./CreateTaskDialog";

const TASK_TYPE_META: Record<string, { icon: React.FC<any>; color: string; bg: string; label: string }> = {
    endpoint: { icon: LucideGlobe, color: "text-sky-400", bg: "bg-sky-500/10", label: "Endpoint" },
    function: { icon: LucideCode, color: "text-violet-400", bg: "bg-violet-500/10", label: "Function" },
    script: { icon: LucideTerminal, color: "text-amber-400", bg: "bg-amber-500/10", label: "Script" },
};

interface AgentDetailPanelProps {
    agent: any;
    onClose: () => void;
}

export function AgentDetailPanel({ agent, onClose }: AgentDetailPanelProps) {
    const [tasks, setTasks] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [showCreateDialog, setShowCreateDialog] = useState(false);
    const [copiedId, setCopiedId] = useState<string | null>(null);

    const fetchTasks = async () => {
        setLoading(true);
        try {
            const res = await fetch(`http://localhost:8080/tasks/agent/${agent.id}`);
            if (res.ok) {
                setTasks(await res.json());
            }
        } catch (err) {
            console.error("Error fetching tasks", err);
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchTasks();
    }, [agent.id]);

    const handleDelete = async (taskId: string) => {
        if (!confirm("Are you sure you want to delete this task?")) return;
        try {
            const res = await fetch(`http://localhost:8080/tasks/${taskId}`, { method: "DELETE" });
            if (res.ok) fetchTasks();
        } catch (err) {
            console.error("Error deleting task", err);
        }
    };

    const copyToClipboard = (text: string, id: string) => {
        navigator.clipboard.writeText(text);
        setCopiedId(id);
        setTimeout(() => setCopiedId(null), 2000);
    };

    const renderStatusDot = (status: string) => {
        const colors: Record<string, string> = {
            Ready: "bg-emerald-500 shadow-[0_0_6px_rgba(16,185,129,0.6)]",
            Pending: "bg-amber-500",
            Unreachable: "bg-rose-500 shadow-[0_0_6px_rgba(244,63,94,0.6)]",
        };
        return <span className={`w-2.5 h-2.5 rounded-full inline-block ${colors[status] || "bg-gray-500"}`}></span>;
    };

    return (
        <>
            {/* Overlay */}
            <div className="fixed inset-0 z-[150] flex justify-end pointer-events-auto">
                <div className="absolute inset-0 bg-black/40 backdrop-blur-sm" onClick={onClose}></div>

                {/* Slide-in Panel */}
                <div className="relative z-10 w-full max-w-xl bg-[#0d0e12] border-l border-gray-800 shadow-2xl flex flex-col h-full animate-slide-in">
                    {/* Panel Header */}
                    <div className="p-6 border-b border-gray-800 bg-[#0b0c10]/80">
                        <div className="flex items-start justify-between mb-4">
                            <div className="flex items-center gap-3">
                                <div className="bg-indigo-500/10 border border-indigo-500/20 p-2.5 rounded-xl">
                                    <LucideServer size={22} className="text-indigo-400" />
                                </div>
                                <div>
                                    <h2 className="text-xl font-bold text-white capitalize tracking-tight">{agent.slug.replace(/-/g, " ")}</h2>
                                    <div className="flex items-center gap-2 mt-1">
                                        {renderStatusDot(agent.status)}
                                        <span className="text-xs text-gray-400">{agent.status}</span>
                                    </div>
                                </div>
                            </div>
                            <button onClick={onClose} className="text-gray-500 hover:text-white transition-colors bg-gray-800/30 p-1.5 rounded-lg border border-gray-800/60">
                                <LucideX size={18} />
                            </button>
                        </div>

                        {/* Agent Info Row */}
                        <div className="grid grid-cols-2 gap-3">
                            <div className="bg-[#111318]/80 border border-gray-800/60 rounded-xl p-3">
                                <span className="text-[10px] uppercase tracking-widest font-bold text-gray-600 block mb-1">Base URL</span>
                                <span className="text-indigo-400 text-sm font-mono truncate block">{agent.endpoint}</span>
                            </div>
                            <div className="bg-[#111318]/80 border border-gray-800/60 rounded-xl p-3">
                                <div className="flex items-center justify-between">
                                    <span className="text-[10px] uppercase tracking-widest font-bold text-gray-600 block mb-1">Agent ID</span>
                                    <button
                                        onClick={() => copyToClipboard(agent.id, "agent-id")}
                                        className="text-gray-600 hover:text-gray-300 transition-colors"
                                        title="Copy"
                                    >
                                        <LucideClipboard size={12} />
                                    </button>
                                </div>
                                <span className="text-gray-300 text-xs font-mono truncate block">
                                    {copiedId === "agent-id" ? "Copied!" : agent.id.slice(0, 16) + "..."}
                                </span>
                            </div>
                        </div>
                    </div>

                    {/* Tasks Section */}
                    <div className="flex-1 overflow-y-auto p-6 custom-scrollbar">
                        <div className="flex items-center justify-between mb-4">
                            <h3 className="text-sm font-bold text-gray-300 uppercase tracking-wider flex items-center gap-2">
                                Tasks
                                <span className="text-[10px] bg-gray-800 text-gray-400 px-1.5 py-0.5 rounded-md font-mono">{tasks.length}</span>
                            </h3>
                            <button
                                onClick={() => setShowCreateDialog(true)}
                                className="text-xs font-semibold text-emerald-400 bg-emerald-500/10 border border-emerald-500/20 px-3 py-1.5 rounded-lg flex items-center gap-1.5 hover:bg-emerald-500/20 transition-colors"
                            >
                                <LucidePlus size={14} /> New Task
                            </button>
                        </div>

                        {loading ? (
                            <div className="flex justify-center py-12">
                                <div className="w-6 h-6 border-2 border-indigo-500 border-t-transparent rounded-full animate-spin"></div>
                            </div>
                        ) : tasks.length === 0 ? (
                            <div className="text-center py-16 border border-gray-800/60 border-dashed rounded-2xl bg-gray-900/20">
                                <div className="bg-gray-800/40 w-12 h-12 rounded-full flex items-center justify-center mx-auto mb-3">
                                    <LucidePlus size={24} className="text-gray-600" />
                                </div>
                                <p className="text-gray-500 text-sm">No tasks registered for this agent.</p>
                                <p className="text-gray-600 text-xs mt-1">Click "New Task" to define a service.</p>
                            </div>
                        ) : (
                            <div className="space-y-3">
                                {tasks.map(task => {
                                    const meta = TASK_TYPE_META[task.task_type] || TASK_TYPE_META.endpoint;
                                    const Icon = meta.icon;

                                    return (
                                        <div key={task.id} className="bg-[#111318] border border-gray-800/80 rounded-xl overflow-hidden hover:border-gray-700/80 transition-all group">
                                            <div className="p-4">
                                                <div className="flex items-start justify-between mb-3">
                                                    <div className="flex items-center gap-2.5">
                                                        <div className={`${meta.bg} border border-current/10 p-1.5 rounded-lg ${meta.color}`}>
                                                            <Icon size={16} />
                                                        </div>
                                                        <div>
                                                            <h4 className="text-sm font-bold text-gray-100">{task.name}</h4>
                                                            {task.description && <p className="text-xs text-gray-500 mt-0.5 line-clamp-1">{task.description}</p>}
                                                        </div>
                                                    </div>
                                                    <div className="flex items-center gap-1.5 opacity-0 group-hover:opacity-100 transition-opacity">
                                                        <button
                                                            onClick={() => copyToClipboard(task.id, task.id)}
                                                            className="text-gray-500 hover:text-gray-300 p-1.5 rounded-lg hover:bg-gray-800/50 transition-all"
                                                            title="Copy Task ID"
                                                        >
                                                            <LucideClipboard size={14} />
                                                        </button>
                                                        <button
                                                            onClick={() => handleDelete(task.id)}
                                                            className="text-gray-500 hover:text-rose-400 p-1.5 rounded-lg hover:bg-rose-500/10 transition-all"
                                                            title="Delete Task"
                                                        >
                                                            <LucideTrash size={14} />
                                                        </button>
                                                    </div>
                                                </div>

                                                {/* Task metadata row */}
                                                <div className="flex flex-wrap gap-2">
                                                    <span className={`inline-flex items-center gap-1 px-2 py-0.5 rounded-md text-[10px] font-bold uppercase tracking-wider ${meta.bg} border border-current/10 ${meta.color}`}>
                                                        {meta.label}
                                                    </span>
                                                    {task.method && (
                                                        <span className="inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-bold uppercase tracking-wider bg-gray-800/50 text-gray-400 border border-gray-700/30 font-mono">
                                                            {task.method}
                                                        </span>
                                                    )}
                                                    {task.path && (
                                                        <span className="inline-flex items-center px-2 py-0.5 rounded-md text-[10px] font-mono text-gray-400 bg-gray-800/50 border border-gray-700/30 max-w-[200px] truncate">
                                                            {task.path}
                                                        </span>
                                                    )}
                                                </div>

                                                {/* Resolved URL preview */}
                                                {task.task_type === "endpoint" && task.path && (
                                                    <div className="mt-3 bg-[#0b0c10] border border-gray-800/40 rounded-lg px-3 py-2">
                                                        <span className="text-[10px] uppercase tracking-widest font-bold text-gray-600 block mb-0.5">Resolved URL</span>
                                                        <span className="text-emerald-400/80 text-xs font-mono block truncate">
                                                            {agent.endpoint.replace(/\/$/, "")}{task.path.startsWith("/") ? "" : "/"}{task.path}
                                                        </span>
                                                    </div>
                                                )}

                                                {copiedId === task.id && (
                                                    <div className="mt-2 text-[10px] text-emerald-400 font-semibold">ID copied to clipboard!</div>
                                                )}
                                            </div>
                                        </div>
                                    );
                                })}
                            </div>
                        )}
                    </div>
                </div>
            </div>

            {showCreateDialog && (
                <CreateTaskDialog
                    agentId={agent.id}
                    onClose={() => setShowCreateDialog(false)}
                    onSuccess={() => {
                        setShowCreateDialog(false);
                        fetchTasks();
                    }}
                />
            )}

            <style>{`
            @keyframes slideIn { from { transform: translateX(100%); opacity: 0; } to { transform: translateX(0); opacity: 1; } }
            .animate-slide-in { animation: slideIn 0.25s ease-out; }
            .custom-scrollbar::-webkit-scrollbar { width: 5px; }
            .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
            .custom-scrollbar::-webkit-scrollbar-thumb { background-color: #1f2937; border-radius: 10px; }
            `}</style>
        </>
    );
}

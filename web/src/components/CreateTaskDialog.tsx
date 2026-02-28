import React, { useState } from "react";
import { LucideX, LucideGlobe, LucideTerminal, LucideCode } from "./icons";

const TASK_TYPES = [
    { value: "endpoint", label: "HTTP Endpoint", icon: LucideGlobe, color: "text-sky-400", bg: "bg-sky-500/10 border-sky-500/20" },
    { value: "function", label: "Internal Function", icon: LucideCode, color: "text-violet-400", bg: "bg-violet-500/10 border-violet-500/20" },
    { value: "script", label: "Script / CLI", icon: LucideTerminal, color: "text-amber-400", bg: "bg-amber-500/10 border-amber-500/20" },
];

const HTTP_METHODS = ["GET", "POST", "PUT", "PATCH", "DELETE"];

interface CreateTaskDialogProps {
    agentId: string;
    onClose: () => void;
    onSuccess: () => void;
}

export function CreateTaskDialog({ agentId, onClose, onSuccess }: CreateTaskDialogProps) {
    const [name, setName] = useState("");
    const [description, setDescription] = useState("");
    const [taskType, setTaskType] = useState("endpoint");
    const [path, setPath] = useState("");
    const [method, setMethod] = useState("POST");
    const [saving, setSaving] = useState(false);

    const handleSave = async () => {
        if (!name.trim()) return alert("Please provide a task name.");
        if (taskType === "endpoint" && !path.trim()) return alert("Please provide an endpoint path.");

        setSaving(true);
        try {
            const res = await fetch("http://localhost:8080/tasks", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({
                    agent_id: agentId,
                    name: name.trim(),
                    description: description.trim() || null,
                    task_type: taskType,
                    path: taskType === "endpoint" ? path.trim() : null,
                    method: taskType === "endpoint" ? method : null,
                    input_contract: null,
                    output_contract: null,
                    settings: null,
                }),
            });
            if (!res.ok) throw new Error(await res.text());
            onSuccess();
        } catch (err) {
            console.error(err);
            alert("Error creating task. Check console.");
        } finally {
            setSaving(false);
        }
    };

    return (
        <div className="fixed inset-0 z-[200] flex items-center justify-center pointer-events-auto">
            <div className="absolute inset-0 bg-black/60 backdrop-blur-sm" onClick={onClose}></div>
            <div className="bg-[#111318] border border-gray-800 rounded-2xl w-full max-w-md shadow-2xl overflow-hidden flex flex-col max-h-[85vh] relative z-10 m-4">
                {/* Header */}
                <div className="flex items-center justify-between p-5 border-b border-gray-800 bg-[#0b0c10]/50">
                    <h2 className="text-lg font-bold text-gray-100 flex items-center gap-2">
                        <span className="w-2 h-2 rounded-full shadow-[0_0_8px_rgba(34,197,94,0.8)] bg-emerald-500"></span>
                        New Task
                    </h2>
                    <button onClick={onClose} className="text-gray-400 hover:text-white transition-colors bg-gray-800/30 p-1.5 rounded-lg border border-gray-800/60"><LucideX size={18} /></button>
                </div>

                {/* Body */}
                <div className="p-5 overflow-y-auto flex-1 space-y-5 custom-scrollbar">
                    {/* Task Name */}
                    <div>
                        <label className="block text-[10px] uppercase tracking-widest font-bold text-gray-500 mb-1.5">Task Name</label>
                        <input
                            type="text" value={name} onChange={e => setName(e.target.value)}
                            className="w-full bg-[#0b0c10] border border-gray-800 rounded-lg px-3.5 py-2.5 text-white text-sm focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all placeholder:text-gray-600"
                            placeholder="e.g. Generate Text"
                        />
                    </div>

                    {/* Description */}
                    <div>
                        <label className="block text-[10px] uppercase tracking-widest font-bold text-gray-500 mb-1.5">Description</label>
                        <textarea
                            value={description} onChange={e => setDescription(e.target.value)}
                            className="w-full bg-[#0b0c10] border border-gray-800 rounded-lg px-3.5 py-2.5 text-white text-sm focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all resize-none h-16 placeholder:text-gray-600"
                            placeholder="What does this task do?"
                        />
                    </div>

                    {/* Task Type Selector */}
                    <div>
                        <label className="block text-[10px] uppercase tracking-widest font-bold text-gray-500 mb-2">Task Type</label>
                        <div className="grid grid-cols-3 gap-2">
                            {TASK_TYPES.map(tt => {
                                const Icon = tt.icon;
                                const isActive = taskType === tt.value;
                                return (
                                    <button
                                        key={tt.value}
                                        onClick={() => setTaskType(tt.value)}
                                        className={`flex flex-col items-center gap-1.5 p-3 rounded-xl border text-xs font-semibold transition-all cursor-pointer ${isActive ? `${tt.bg} ${tt.color} ring-1 ring-current` : "bg-gray-900/30 border-gray-800 text-gray-500 hover:border-gray-700 hover:text-gray-400"}`}
                                    >
                                        <Icon size={20} />
                                        {tt.label}
                                    </button>
                                );
                            })}
                        </div>
                    </div>

                    {/* Endpoint-specific fields */}
                    {taskType === "endpoint" && (
                        <div className="space-y-4 pt-1">
                            <div className="flex gap-2">
                                <div className="w-28 shrink-0">
                                    <label className="block text-[10px] uppercase tracking-widest font-bold text-gray-500 mb-1.5">Method</label>
                                    <select
                                        value={method} onChange={e => setMethod(e.target.value)}
                                        className="w-full bg-[#0b0c10] border border-gray-800 rounded-lg px-3 py-2.5 text-sm text-white focus:outline-none focus:border-indigo-500 appearance-none"
                                    >
                                        {HTTP_METHODS.map(m => <option key={m} value={m} className="bg-[#111318]">{m}</option>)}
                                    </select>
                                </div>
                                <div className="flex-1">
                                    <label className="block text-[10px] uppercase tracking-widest font-bold text-gray-500 mb-1.5">Path</label>
                                    <input
                                        type="text" value={path} onChange={e => setPath(e.target.value)}
                                        className="w-full bg-[#0b0c10] border border-gray-800 rounded-lg px-3.5 py-2.5 text-white text-sm focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all placeholder:text-gray-600 font-mono"
                                        placeholder="/api/generate"
                                    />
                                </div>
                            </div>
                        </div>
                    )}
                </div>

                {/* Footer */}
                <div className="p-4 border-t border-gray-800 bg-[#0a0a0c] flex justify-end gap-3 shrink-0">
                    <button onClick={onClose} className="px-4 py-2 text-sm font-semibold text-gray-400 hover:text-white hover:bg-gray-800 rounded-lg transition-colors">Cancel</button>
                    <button
                        onClick={handleSave} disabled={saving}
                        className="bg-emerald-600 hover:bg-emerald-500 disabled:bg-gray-800 disabled:text-gray-500 disabled:cursor-not-allowed text-white text-sm font-semibold py-2 px-5 rounded-lg transition-all shadow-[0_0_10px_rgba(16,185,129,0.2)]"
                    >
                        {saving ? "Creating..." : "Create Task"}
                    </button>
                </div>
            </div>

            <style>{`
            .custom-scrollbar::-webkit-scrollbar { width: 5px; }
            .custom-scrollbar::-webkit-scrollbar-track { background: transparent; }
            .custom-scrollbar::-webkit-scrollbar-thumb { background-color: #1f2937; border-radius: 10px; }
            `}</style>
        </div>
    );
}

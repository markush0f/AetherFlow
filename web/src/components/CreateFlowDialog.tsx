import React, { useState, useEffect } from "react";
import { LucidePlus, LucideTrash, LucideX, LucideGlobe, LucideCode, LucideTerminal } from "./icons";

const TASK_TYPE_ICONS: Record<string, React.FC<any>> = {
    endpoint: LucideGlobe,
    function: LucideCode,
    script: LucideTerminal,
};

const TASK_TYPE_COLORS: Record<string, string> = {
    endpoint: "text-sky-400",
    function: "text-violet-400",
    script: "text-amber-400",
};

interface TaskOption {
    id: string;
    agent_id: string;
    name: string;
    task_type: string;
    agentSlug?: string;
}

export function CreateFlowDialog({ onClose, onSuccess }: { onClose: () => void; onSuccess: () => void }) {
    const [name, setName] = useState("");
    const [description, setDescription] = useState("");
    const [tasks, setTasks] = useState<TaskOption[]>([]);
    const [steps, setSteps] = useState<string[]>([]);
    const [saving, setSaving] = useState(false);

    useEffect(() => {
        const fetchData = async () => {
            try {
                const [agentsRes, tasksRes] = await Promise.all([
                    fetch("http://localhost:8080/agents"),
                    fetch("http://localhost:8080/tasks"),
                ]);
                const agents = await agentsRes.json();
                const allTasks = await tasksRes.json();

                // Map agent slug to each task for display
                const agentMap: Record<string, string> = {};
                agents.forEach((a: any) => { agentMap[a.id] = a.slug; });
                const enrichedTasks = allTasks.map((t: any) => ({
                    ...t,
                    agentSlug: agentMap[t.agent_id] || "unknown",
                }));
                enrichedTasks.sort((a: any, b: any) => a.agentSlug.localeCompare(b.agentSlug));
                setTasks(enrichedTasks);
            } catch (err) {
                console.error("Error fetching data", err);
            }
        };
        fetchData();
    }, []);

    const handleSave = async () => {
        if (!name) return alert("Please provide a name for the flow.");
        if (steps.length === 0) return alert("A flow must have at least one step.");
        if (steps.some(s => s === "")) return alert("Please select a task for all steps or remove the empty step.");

        setSaving(true);
        try {
            // 1. Create Flow
            const flowRes = await fetch("http://localhost:8080/flows", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ name, description })
            });
            if (!flowRes.ok) throw new Error("Failed to create flow");
            const newFlow = await flowRes.json();
            const flowId = newFlow.id;

            // 2. Create Flow Steps (now referencing task_id)
            for (let i = 0; i < steps.length; i++) {
                const stepRes = await fetch(`http://localhost:8080/flows/${flowId}/steps`, {
                    method: "POST",
                    headers: { "Content-Type": "application/json" },
                    body: JSON.stringify({
                        task_id: steps[i],
                        step_order: i + 1
                    })
                });
                if (!stepRes.ok) {
                    console.error(await stepRes.text());
                    throw new Error(`Failed to create step ${i + 1}`);
                }
            }

            onSuccess();
        } catch (error) {
            console.error(error);
            alert("Error creating flow. Check console.");
        } finally {
            setSaving(false);
        }
    };

    return (
        <div className="fixed inset-0 z-[100] flex items-center justify-center pointer-events-auto">
            <div className="absolute inset-0 bg-black/50 backdrop-blur-sm" onClick={onClose}></div>
            <div className="bg-[#111318] border border-gray-800 rounded-2xl w-full max-w-lg shadow-2xl overflow-hidden flex flex-col max-h-[90vh] relative z-10 m-4">
                <div className="flex items-center justify-between p-6 border-b border-gray-800 bg-[#0b0c10]/50">
                    <h2 className="text-xl font-bold text-gray-100 flex items-center gap-2">
                        <span className="w-2 h-2 rounded-full shadow-[0_0_8px_rgba(99,102,241,0.8)] bg-indigo-500"></span>
                        Create New Flow
                    </h2>
                    <button onClick={onClose} className="text-gray-400 hover:text-white transition-colors bg-gray-800/30 p-1.5 rounded-lg border border-gray-800/60"><LucideX size={20} /></button>
                </div>

                <div className="p-6 overflow-y-auto flex-1 custom-scrollbar">
                    <div className="mb-4">
                        <label className="block text-xs uppercase tracking-wider font-bold text-gray-400 mb-2">Flow Name</label>
                        <input
                            type="text"
                            value={name}
                            onChange={(e) => setName(e.target.value)}
                            className="w-full bg-[#0b0c10] border border-gray-800 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all text-sm placeholder:text-gray-600"
                            placeholder="e.g. Data Ingestion Pipeline"
                        />
                    </div>

                    <div className="mb-8">
                        <label className="block text-xs uppercase tracking-wider font-bold text-gray-400 mb-2">Description</label>
                        <textarea
                            value={description}
                            onChange={(e) => setDescription(e.target.value)}
                            className="w-full bg-[#0b0c10] border border-gray-800 rounded-lg px-4 py-2.5 text-white focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 transition-all h-24 resize-none text-sm placeholder:text-gray-600"
                            placeholder="Briefly describe what this flow does..."
                        />
                    </div>

                    <div className="mb-3 flex items-center justify-between">
                        <label className="block text-xs uppercase tracking-wider font-bold text-gray-400">Task Sequence</label>
                        <button
                            onClick={() => setSteps([...steps, ""])}
                            className="text-xs font-semibold text-indigo-400 bg-indigo-500/10 border border-indigo-500/20 px-2.5 py-1.5 rounded-lg flex items-center gap-1.5 hover:bg-indigo-500/20 hover:text-indigo-300 transition-colors shadow-sm"
                        >
                            <LucidePlus size={14} /> Add Step
                        </button>
                    </div>

                    <div className="space-y-3">
                        {steps.length === 0 ? (
                            <div className="text-center p-8 border border-gray-800 border-dashed rounded-xl bg-gray-900/20 text-gray-500 text-sm flex flex-col items-center">
                                <div className="bg-gray-800/50 p-3 rounded-full mb-3 text-gray-600">
                                    <LucidePlus size={24} />
                                </div>
                                <p>No tasks added yet.</p>
                                <p className="text-xs text-gray-600 mt-1">Click "Add Step" to build the pipeline.</p>
                            </div>
                        ) : (
                            steps.map((stepTaskId, idx) => {
                                const selectedTask = tasks.find(t => t.id === stepTaskId);
                                return (
                                    <div key={idx} className="flex gap-3 items-center bg-[#13151a] border border-gray-800/80 p-3.5 rounded-xl shadow-sm hover:border-gray-700 transition-colors">
                                        <div className="bg-indigo-500/10 text-indigo-400 border border-indigo-500/20 px-2 py-1.5 rounded-lg text-[10px] font-mono shrink-0 uppercase tracking-wider font-bold">
                                            Step_0{idx + 1}
                                        </div>
                                        <select
                                            value={stepTaskId}
                                            onChange={(e) => {
                                                const newSteps = [...steps];
                                                newSteps[idx] = e.target.value;
                                                setSteps(newSteps);
                                            }}
                                            className="grow bg-[#0b0c10] border border-gray-800 rounded-lg px-3 py-2 text-sm text-gray-100 focus:outline-none focus:border-indigo-500 focus:ring-1 focus:ring-indigo-500 appearance-none"
                                            style={{ backgroundImage: `url('data:image/svg+xml;utf8,<svg xmlns="http://www.w3.org/2000/svg" fill="none" viewBox="0 0 24 24" stroke="%236b7280"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 9l-7 7-7-7"></path></svg>')`, backgroundRepeat: 'no-repeat', backgroundPosition: 'right 0.75rem center', backgroundSize: '1em' }}
                                        >
                                            <option value="" disabled className="text-gray-500">Select a task...</option>
                                            {tasks.map(task => (
                                                <option key={task.id} value={task.id} className="bg-[#111318]">
                                                    [{task.agentSlug}] {task.name} ({task.task_type})
                                                </option>
                                            ))}
                                        </select>
                                        <button
                                            onClick={() => {
                                                const newSteps = [...steps];
                                                newSteps.splice(idx, 1);
                                                setSteps(newSteps);
                                            }}
                                            className="text-gray-500 hover:text-rose-400 hover:bg-rose-400/10 p-2 rounded-lg border border-transparent hover:border-rose-400/20 transition-all shrink-0"
                                            title="Remove step"
                                        >
                                            <LucideTrash size={16} />
                                        </button>
                                    </div>
                                );
                            })
                        )}
                    </div>
                </div>

                <div className="p-5 border-t border-gray-800 bg-[#0a0a0c] flex justify-end gap-3 shrink-0">
                    <button
                        onClick={onClose}
                        className="px-4 py-2 text-sm font-semibold text-gray-400 hover:text-white hover:bg-gray-800 rounded-lg transition-colors border border-transparent"
                    >
                        Cancel
                    </button>
                    <button
                        onClick={handleSave}
                        disabled={saving}
                        className="bg-indigo-600 hover:bg-indigo-500 disabled:bg-gray-800 disabled:text-gray-500 disabled:cursor-not-allowed text-white text-sm font-semibold py-2 px-6 rounded-lg transition-all shadow-[0_0_10px_rgba(79,70,229,0.2)]"
                    >
                        {saving ? "Creating Flow..." : "Save Flow"}
                    </button>
                </div>
            </div>

            <style>{`
            .custom-scrollbar::-webkit-scrollbar {
                width: 6px;
            }
            .custom-scrollbar::-webkit-scrollbar-track {
                background: transparent;
            }
            .custom-scrollbar::-webkit-scrollbar-thumb {
                background-color: #1f2937;
                border-radius: 10px;
            }
            `}</style>
        </div>
    );
}

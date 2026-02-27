import React, { useState, useEffect } from 'react';
import FlowEditor from './FlowEditor';
import { CreateFlowDialog } from './CreateFlowDialog';
import { LucideSettings, LucideArrowRight, LucideLink, LucidePlus } from './icons';

export default function FlowGrid() {
    const [flows, setFlows] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [selectedFlow, setSelectedFlow] = useState<any>(null);

    const [showCreateDialog, setShowCreateDialog] = useState(false);

    const fetchFlows = async () => {
        setLoading(true);
        try {
            const response = await fetch("http://localhost:8080/flows");
            if (!response.ok) throw new Error("Failed fetching flows");
            const data = await response.json();
            setFlows(data);
        } catch (err) {
            console.error("Error API:", err);
            setError("Failed to fetch flows from backend. Is the server running?");
        } finally {
            setLoading(false);
        }
    };

    useEffect(() => {
        fetchFlows();
    }, []);

    const executeFlow = async (id: string, e: React.MouseEvent) => {
        e.stopPropagation(); // prevent opening the flow editor when clicking execute
        if (!window.confirm("Are you sure you want to trigger this execution with an empty initial payload?")) return;

        try {
            const response = await fetch(`http://localhost:8080/flows/${id}/execute`, {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ payload: {} }),
            });
            const result = await response.json();
            alert("Flow executed! Final result: \n" + JSON.stringify(result, null, 2));
        } catch (error) {
            alert("Execution failed! See console.");
            console.error(error);
        }
    };

    if (loading) {
        return (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div className="col-span-full flex justify-center py-20 rounded-lg bg-[#1e2124] border border-gray-700/50">
                    <div className="animate-pulse flex flex-col items-center">
                        <div className="w-8 h-8 border-2 border-blue-500 border-t-transparent rounded-full animate-spin mb-4"></div>
                        <p className="text-gray-400 text-sm">Loading data from postgres...</p>
                    </div>
                </div>
            </div>
        );
    }

    if (error) {
        return (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div className="col-span-full py-12 text-center bg-red-900/10 rounded-3xl border border-red-500/20 backdrop-blur-md">
                    <p className="text-red-400">{error}</p>
                </div>
            </div>
        );
    }

    if (flows.length === 0) {
        return (
            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                <div className="col-span-full py-12 text-center bg-gray-800/20 rounded-2xl border border-gray-800 border-dashed">
                    <p className="text-gray-400">No flows found in the database.</p>
                </div>
            </div>
        );
    }

    return (
        <>
            {selectedFlow && (
                <FlowEditor flow={selectedFlow} onBack={() => setSelectedFlow(null)} />
            )}
            {showCreateDialog && (
                <CreateFlowDialog
                    onClose={() => setShowCreateDialog(false)}
                    onSuccess={() => { setShowCreateDialog(false); fetchFlows(); }}
                />
            )}

            <div className="flex justify-end mb-6">
                <button
                    onClick={() => setShowCreateDialog(true)}
                    className="bg-indigo-600 hover:bg-indigo-500 text-white font-medium py-2 px-5 rounded-lg transition-all flex items-center gap-2 text-sm shadow-[0_0_15px_rgba(79,70,229,0.3)] hover:shadow-[0_0_20px_rgba(79,70,229,0.5)] active:scale-95 border border-indigo-400/20"
                >
                    <LucidePlus className="w-4 h-4" />
                    Create New Flow
                </button>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-6">
                {flows.map((flow) => (
                    <div
                        key={flow.id}
                        onClick={() => setSelectedFlow(flow)}
                        className="cursor-pointer bg-[#111318] border border-gray-800 rounded-2xl shadow-sm hover:border-indigo-500/40 hover:shadow-[0_4px_20px_-4px_rgba(99,102,241,0.1)] transition-all duration-300 flex flex-col h-full overflow-hidden"
                    >
                        <div className="p-6 grow flex flex-col">
                            <div className="flex items-center gap-3 mb-5">
                                <div className="bg-indigo-500/10 text-indigo-400 p-2 rounded-lg border border-indigo-500/20">
                                    <LucideLink className="w-5 h-5" />
                                </div>
                                <h3 className="text-xl font-bold text-gray-100 tracking-tight">{flow.name}</h3>
                            </div>

                            <p className="text-gray-400 text-sm mb-6 leading-relaxed">
                                {flow.description || "No description provided."}
                            </p>

                            <div className="bg-gray-800/30 p-3 rounded-xl border border-gray-800/60 mb-6">
                                <span className="text-gray-500 text-[10px] uppercase font-bold tracking-wider block mb-1">Flow UUID</span>
                                <span className="font-mono text-xs text-gray-300 block truncate">{flow.id}</span>
                            </div>

                            <div className="mt-auto pt-4 border-t border-gray-800/60">
                                <span className="text-gray-500 text-[10px] uppercase font-bold tracking-wider block mb-3">Agent Sequence</span>
                                {flow.agents_chain && flow.agents_chain.length > 0 ? (
                                    <div className="flex flex-wrap items-center gap-2">
                                        {flow.agents_chain.map((slug: string, idx: number) => (
                                            <div key={idx} className="flex items-center gap-2">
                                                <span className="bg-indigo-500/10 text-indigo-400 border border-indigo-500/20 px-2.5 py-1 rounded text-xs font-semibold tracking-wide capitalize">
                                                    {slug.replace(/-/g, " ")}
                                                </span>
                                                {idx < flow.agents_chain.length - 1 && (
                                                    <LucideArrowRight className="w-4 h-4 text-gray-600" />
                                                )}
                                            </div>
                                        ))}
                                    </div>
                                ) : (
                                    <span className="text-gray-500 text-sm italic">No agents connected to this pipeline yet.</span>
                                )}
                            </div>
                        </div>

                        <div className="px-6 py-4 bg-gray-900/50 border-t border-gray-800 flex gap-3">
                            <button className="flex-1 bg-white hover:bg-gray-200 text-gray-900 font-semibold py-2 px-4 rounded-xl text-sm transition-all shadow-sm active:scale-[0.98]" onClick={(e) => executeFlow(flow.id, e)}>
                                Trigger Execute
                            </button>
                            <button className="flex-none bg-[#111318] hover:bg-gray-800 text-gray-400 hover:text-white p-2 border border-gray-800 rounded-xl transition-all" title="Manage Config" onClick={(e) => { e.stopPropagation(); setSelectedFlow(flow); }}>
                                <LucideSettings className="w-5 h-5 mx-auto" />
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </>
    );
}

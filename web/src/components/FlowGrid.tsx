import React, { useState, useEffect } from 'react';
import FlowEditor from './FlowEditor';

export default function FlowGrid() {
    const [flows, setFlows] = useState<any[]>([]);
    const [loading, setLoading] = useState(true);
    const [error, setError] = useState<string | null>(null);
    const [selectedFlow, setSelectedFlow] = useState<any>(null);

    useEffect(() => {
        async function fetchFlows() {
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
        }

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
                                    <svg className="w-5 h-5" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M13.828 10.172a4 4 0 00-5.656 0l-4 4a4 4 0 105.656 5.656l1.102-1.101m-.758-4.899a4 4 0 005.656 0l4-4a4 4 0 00-5.656-5.656l-1.1 1.1"></path></svg>
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
                                                    <svg className="w-4 h-4 text-gray-600" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M14 5l7 7m0 0l-7 7m7-7H3"></path></svg>
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
                                <svg className="w-5 h-5 mx-auto" fill="none" viewBox="0 0 24 24" stroke="currentColor"><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.065 2.572c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.572 1.065c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.065-2.572c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z"></path><path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 12a3 3 0 11-6 0 3 3 0 016 0z"></path></svg>
                            </button>
                        </div>
                    </div>
                ))}
            </div>
        </>
    );
}

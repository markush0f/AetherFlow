import React, { useCallback, useState } from 'react';
import ReactFlow, {
    Controls,
    Background,
    applyNodeChanges,
    applyEdgeChanges,
    addEdge,
    Handle,
    Position
} from 'reactflow';
import 'reactflow/dist/style.css';
import { Bot, Play, X } from 'lucide-react';

const AgentNode = ({ data }: any) => {
    return (
        <div className="bg-[#111318] border border-indigo-500/50 rounded-xl shadow-[0_4px_20px_-4px_rgba(99,102,241,0.2)] p-4 min-w-[200px]">
            <Handle type="target" position={Position.Left} className="w-3 h-3 bg-indigo-500 !border-0" />
            <div className="flex items-center gap-3">
                <div className="bg-indigo-500/20 p-2 rounded-lg text-indigo-400">
                    <Bot size={20} />
                </div>
                <div>
                    <div className="text-[10px] text-gray-500 uppercase tracking-widest font-bold mb-0.5">Step {data.stepOrder}</div>
                    <div className="text-sm text-gray-100 font-semibold capitalize tracking-tight">{data.agentSlug.replace(/-/g, ' ')}</div>
                </div>
            </div>
            <Handle type="source" position={Position.Right} className="w-3 h-3 bg-indigo-500 !border-0" />
        </div>
    );
};

const nodeTypes = { agentNode: AgentNode };

export default function FlowEditor({ flow, onBack }: { flow: any, onBack: any }) {
    const [nodes, setNodes] = useState<any[]>([]);
    const [edges, setEdges] = useState<any[]>([]);

    React.useEffect(() => {
        if (!flow || !flow.steps) return;
        const initialNodes = flow.steps.map((step: any, idx: number) => ({
            id: step.id,
            type: 'agentNode',
            position: { x: idx * 350 + 100, y: Math.max(window.innerHeight / 2 - 100, 100) },
            data: { agentSlug: step.agent_slug, stepOrder: step.step_order }
        }));

        const initialEdges = [];
        for (let i = 0; i < initialNodes.length - 1; i++) {
            initialEdges.push({
                id: `e-${initialNodes[i].id}-${initialNodes[i + 1].id}`,
                source: initialNodes[i].id,
                target: initialNodes[i + 1].id,
                animated: true,
                style: { stroke: '#6366f1', strokeWidth: 2 }
            });
        }

        setNodes(initialNodes);
        setEdges(initialEdges);
    }, [flow]);

    const onNodesChange = useCallback((changes: any) => setNodes((nds: any) => applyNodeChanges(changes, nds)), []);
    const onEdgesChange = useCallback((changes: any) => setEdges((eds: any) => applyEdgeChanges(changes, eds)), []);
    const onConnect = useCallback((params: any) => setEdges((eds: any) => addEdge({ ...params, animated: true, style: { stroke: '#6366f1', strokeWidth: 2 } }, eds)), []);

    const executeFlow = async () => {
        if (!window.confirm("Trigger flow execution with empty payload?")) return;
        try {
            const response = await fetch(`http://localhost:8080/flows/${flow.id}/execute`, {
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

    return (
        <div className="fixed inset-0 z-[100] flex flex-col bg-[#0b0c10]">
            <div className="h-16 border-b border-gray-800/80 bg-[#111318]/90 backdrop-blur-md flex items-center justify-between px-6 shrink-0 relative z-10">
                <div className="flex items-center gap-4">
                    <button onClick={onBack} className="text-gray-400 hover:text-white hover:bg-gray-800 p-2 rounded-lg transition-colors flex items-center justify-center">
                        <X size={20} />
                    </button>
                    <div className="w-px h-6 bg-gray-800 mx-1"></div>
                    <div>
                        <h2 className="text-sm font-bold text-gray-100 flex items-center gap-2">
                            <span className="w-2 h-2 rounded-full bg-indigo-500 animate-pulse"></span>
                            {flow.name}
                        </h2>
                        <p className="text-[10px] text-gray-500 font-mono mt-0.5">{flow.id}</p>
                    </div>
                </div>
                <button onClick={executeFlow} className="bg-indigo-600 hover:bg-indigo-500 text-white font-medium py-1.5 px-4 rounded-lg transition-colors flex items-center text-sm shadow-sm active:scale-[0.98]">
                    <Play size={14} className="mr-2" />
                    Deploy & Execute
                </button>
            </div>
            <div className="grow relative h-screen w-full">
                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    onConnect={onConnect}
                    nodeTypes={nodeTypes}
                    proOptions={{ hideAttribution: true }}
                    fitView
                    className="bg-[#0b0c10]"
                    minZoom={0.2}
                    maxZoom={1.5}
                >
                    <Background color="#1e2124" gap={20} size={2} />
                    <Controls
                        className="bg-[#111318]/90 border border-gray-800 fill-gray-400 backdrop-blur-md rounded-lg overflow-hidden"
                        showInteractive={false}
                    />
                </ReactFlow>
            </div>
        </div>
    );
}

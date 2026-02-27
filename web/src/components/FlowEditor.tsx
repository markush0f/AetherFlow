import React, { useCallback, useState } from 'react';
import ReactFlow, {
    Controls,
    Background,
    applyNodeChanges,
    applyEdgeChanges,
    addEdge,
    Handle,
    Position,
    MarkerType,
    BackgroundVariant
} from 'reactflow';
import 'reactflow/dist/style.css';
import { Bot, Play, X, Zap } from 'lucide-react';

const AgentNode = ({ data }: any) => {
    return (
        <div className="relative group cursor-crosshair">
            {/* Animated Glow effect behind */}
            <div className="absolute -inset-0.5 bg-gradient-to-r from-indigo-500 via-purple-500 to-cyan-500 rounded-lg blur opacity-20 group-hover:opacity-60 transition duration-1000 group-hover:duration-200"></div>

            <div className="relative flex items-center bg-[#0b0c10] border border-gray-800 rounded-lg overflow-hidden min-w-[260px] shadow-2xl">
                {/* Cyberpunk left edge */}
                <div className="w-1 h-full bg-indigo-500 absolute left-0 top-0"></div>

                <Handle type="target" position={Position.Left} className="w-2 h-8 rounded-sm bg-indigo-500 !border-0 -ml-1 transition-all group-hover:-ml-2" />

                <div className="flex items-center p-3 pl-5 gap-4 w-full backdrop-blur-xl bg-[#13151a]/90">
                    <div className="flex-shrink-0 bg-gray-900 shadow-inner p-2.5 rounded-lg border border-gray-800 text-indigo-400 relative">
                        <Bot size={20} className="drop-shadow-[0_0_8px_rgba(99,102,241,0.5)]" />
                        <div className="absolute -top-1 -right-1 w-2.5 h-2.5 rounded-full bg-emerald-500 border border-[#1e2124] animate-pulse"></div>
                    </div>

                    <div className="flex flex-col grow">
                        <div className="flex justify-between items-center mb-0.5">
                            <div className="text-[9px] text-indigo-500 font-mono tracking-widest uppercase">Seq_0{data.stepOrder}</div>
                            <Zap size={10} className="text-purple-500 opacity-50" />
                        </div>
                        <div className="text-sm text-gray-100 font-semibold tracking-wide uppercase">{data.agentSlug.replace(/-/g, ' ')}</div>
                    </div>
                </div>

                <Handle type="source" position={Position.Right} className="w-2 h-8 rounded-sm bg-purple-500 !border-0 -mr-1 transition-all group-hover:-mr-2" />
            </div>
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
            position: { x: idx * 400 + 100, y: Math.max(window.innerHeight / 2 - 100, 100) },
            data: { agentSlug: step.agent_slug, stepOrder: step.step_order }
        }));

        const initialEdges = [];
        for (let i = 0; i < initialNodes.length - 1; i++) {
            initialEdges.push({
                id: `e-${initialNodes[i].id}-${initialNodes[i + 1].id}`,
                source: initialNodes[i].id,
                target: initialNodes[i + 1].id,
                type: 'step', // 'smoothstep', 'step', or 'straight'
                animated: true,
                style: { stroke: '#8b5cf6', strokeWidth: 2, strokeDasharray: '5, 5' },
                markerEnd: {
                    type: MarkerType.ArrowClosed,
                    width: 15,
                    height: 15,
                    color: '#8b5cf6',
                },
            });
        }

        setNodes(initialNodes);
        setEdges(initialEdges);
    }, [flow]);

    const onNodesChange = useCallback((changes: any) => setNodes((nds: any) => applyNodeChanges(changes, nds)), []);
    const onEdgesChange = useCallback((changes: any) => setEdges((eds: any) => applyEdgeChanges(changes, eds)), []);
    const onConnect = useCallback((params: any) => setEdges((eds: any) => addEdge({
        ...params,
        type: 'step',
        animated: true,
        style: { stroke: '#8b5cf6', strokeWidth: 2, strokeDasharray: '5, 5' },
        markerEnd: { type: MarkerType.ArrowClosed, color: '#8b5cf6' }
    }, eds)), []);

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
        <div className="fixed inset-0 z-[100] flex flex-col bg-[#050507]">
            {/* Header */}
            <div className="h-16 border-b border-gray-800/80 bg-[#0a0a0c]/90 backdrop-blur-xl flex items-center justify-between px-6 shrink-0 relative z-10 shadow-lg">
                <div className="flex items-center gap-5">
                    <button onClick={onBack} className="text-gray-400 hover:text-white hover:bg-gray-800 p-2 rounded-xl transition-all active:scale-95 flex items-center justify-center">
                        <X size={20} />
                    </button>
                    <div className="w-px h-6 bg-gray-800"></div>
                    <div>
                        <h2 className="text-sm font-bold text-gray-100 flex items-center gap-2">
                            <span className="w-2 h-2 rounded-full shadow-[0_0_8px_rgba(139,92,246,0.8)] bg-purple-500"></span>
                            {flow.name}
                        </h2>
                        <p className="text-[10px] text-gray-500 font-mono mt-0.5 opacity-70">UUID // {flow.id}</p>
                    </div>
                </div>
                <button onClick={executeFlow} className="bg-gradient-to-r from-indigo-600 to-purple-600 hover:from-indigo-500 hover:to-purple-500 text-white font-medium py-1.5 px-5 rounded-lg transition-all flex items-center text-sm shadow-[0_0_15px_rgba(79,70,229,0.4)] hover:shadow-[0_0_20px_rgba(79,70,229,0.7)] active:scale-95 border border-indigo-400/20">
                    <Play size={14} className="mr-2 fill-white" />
                    Deploy & Execute
                </button>
            </div>

            {/* Canvas */}
            <div className="grow relative h-screen w-full">
                <div className="absolute inset-0 bg-[radial-gradient(circle_at_center,rgba(99,102,241,0.03)_0%,transparent_100%)] pointer-events-none"></div>
                <ReactFlow
                    nodes={nodes}
                    edges={edges}
                    onNodesChange={onNodesChange}
                    onEdgesChange={onEdgesChange}
                    onConnect={onConnect}
                    nodeTypes={nodeTypes}
                    proOptions={{ hideAttribution: true }}
                    fitView
                    className="bg-transparent"
                    minZoom={0.2}
                    maxZoom={1.5}
                >
                    <Background color="#1a1c23" variant={BackgroundVariant.Lines} gap={40} size={1} />
                    <Controls
                        className="bg-[#111318]/90 border border-gray-800 fill-gray-400 backdrop-blur-md rounded-lg overflow-hidden shadow-xl"
                        showInteractive={false}
                    />
                </ReactFlow>
            </div>
        </div>
    );
}

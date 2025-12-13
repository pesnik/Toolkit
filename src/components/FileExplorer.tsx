'use client';

import * as React from 'react';
import {
    makeStyles,
    shorthands,
    Button,
    Input,
    Text,
    DataGrid,
    DataGridBody,
    DataGridRow,
    DataGridHeader,
    DataGridHeaderCell,
    DataGridCell,
    TableCellLayout,
    TableColumnDefinition,
    createTableColumn,
    ProgressBar,
    Breadcrumb,
    BreadcrumbItem,
    BreadcrumbDivider,
    BreadcrumbButton,
    Avatar,
} from '@fluentui/react-components';
import {
    FolderRegular,
    DocumentRegular,
    ArrowUpRegular,
    ArrowLeftRegular,
    ArrowRightRegular,
    HomeRegular,
    ArrowClockwiseRegular,
    DesktopRegular
} from '@fluentui/react-icons';
import { invoke } from '@tauri-apps/api/core';
import { FileNode } from '@/types';

const useStyles = makeStyles({
    container: {
        display: 'flex',
        flexDirection: 'column',
        height: '100%',
        ...shorthands.gap('10px'),
        ...shorthands.padding('20px'),
    },
    toolbar: {
        display: 'flex',
        alignItems: 'center',
        ...shorthands.gap('8px'),
    },
    pathBar: {
        display: 'flex',
        alignItems: 'center',
        ...shorthands.gap('10px'),
        flexGrow: 1,
    },
    gridContainer: {
        flexGrow: 1,
        overflowY: 'auto',
        ...shorthands.border('1px', 'solid', '#333'), // discrete border
        ...shorthands.borderRadius('4px'),
    },
    statusBar: {
        display: 'flex',
        justifyContent: 'space-between',
        paddingTop: '8px',
        borderTop: '1px solid #333',
    },
});

interface ExplorerState {
    path: string;
    loading: boolean;
    data: FileNode | null;
    history: string[];
    historyIndex: number;
    error: string | null;
}

export const FileExplorer = () => {
    const styles = useStyles();
    const [state, setState] = React.useState<ExplorerState>({
        path: 'C:\\', // Default, will be updated on mount probably
        loading: false,
        data: null,
        history: ['C:\\'],
        historyIndex: 0,
        error: null,
    });

    const [inputPath, setInputPath] = React.useState(state.path);

    // Columns definition
    const columns: TableColumnDefinition<FileNode>[] = [
        createTableColumn({
            columnId: 'file',
            compare: (a, b) => a.name.localeCompare(b.name),
            renderHeaderCell: () => 'Name',
            renderCell: (item) => (
                <TableCellLayout media={item.is_dir ? <FolderRegular /> : <DocumentRegular />}>
                    {item.name}
                </TableCellLayout>
            ),
        }),
        createTableColumn({
            columnId: 'size',
            compare: (a, b) => a.size - b.size,
            renderHeaderCell: () => 'Size',
            renderCell: (item) => formatSize(item.size),
        }),
        createTableColumn({
            columnId: 'count',
            compare: (a, b) => a.file_count - b.file_count,
            renderHeaderCell: () => 'Files',
            renderCell: (item) => item.is_dir ? item.file_count.toLocaleString() : '-',
        }),
        createTableColumn({
            columnId: 'modified',
            compare: (a, b) => a.last_modified - b.last_modified,
            renderHeaderCell: () => 'Modified',
            renderCell: (item) => new Date(item.last_modified * 1000).toLocaleString(),
        }),
    ];

    const fetchData = async (path: string, forceRefresh: boolean = false) => {
        setState(prev => ({ ...prev, loading: true, error: null }));
        try {
            const command = forceRefresh ? 'refresh_scan' : 'scan_dir';
            const data = await invoke<FileNode>(command, { path });
            setState(prev => ({ ...prev, loading: false, data, path }));
            setInputPath(path);
        } catch (e: any) {
            setState(prev => ({ ...prev, loading: false, error: String(e) }));
        }
    };

    React.useEffect(() => {
        const initialPath = '/'; // Default start path
        // Reset history to just this path
        setState(prev => ({
            ...prev,
            history: [initialPath],
            historyIndex: 0,
            path: initialPath,
            loading: true // Show loading initially
        }));
        fetchData(initialPath);
    }, []);

    const handleNavigate = (newPath: string) => {
        if (newPath === state.path) return;

        // Add to history
        const newHistory = state.history.slice(0, state.historyIndex + 1);
        newHistory.push(newPath);

        setState(prev => ({
            ...prev,
            history: newHistory,
            historyIndex: newHistory.length - 1,
        }));

        fetchData(newPath);
    };

    const handleBack = () => {
        if (state.historyIndex > 0) {
            const newIndex = state.historyIndex - 1;
            const prevPath = state.history[newIndex];
            setState(prev => ({ ...prev, historyIndex: newIndex }));
            fetchData(prevPath);
        }
    };

    const handleForward = () => {
        if (state.historyIndex < state.history.length - 1) {
            const newIndex = state.historyIndex + 1;
            const nextPath = state.history[newIndex];
            setState(prev => ({ ...prev, historyIndex: newIndex }));
            fetchData(nextPath);
        }
    };

    const handleUp = () => {
        // Basic navigation up logic
        const separator = state.path.includes('/') ? '/' : '\\';
        const parts = state.path.split(separator).filter(Boolean);
        if (parts.length > 0) {
            parts.pop();
            const parentPath = parts.length === 0 ? '/' : parts.join(separator);
            const finalPath = parentPath === '' ? '/' : (state.path.startsWith('/') ? '/' + parentPath : parentPath);
            handleNavigate(finalPath);
        }
    };

    const items = state.data?.children || [];

    return (
        <div className={styles.container}>
            {/* Toolbar */}
            <div className={styles.toolbar}>
                <Button icon={<ArrowLeftRegular />} disabled={state.historyIndex <= 0} onClick={handleBack} />
                <Button icon={<ArrowRightRegular />} disabled={state.historyIndex >= state.history.length - 1} onClick={handleForward} />
                <Button icon={<ArrowUpRegular />} onClick={handleUp} />
                <Button icon={<ArrowClockwiseRegular />} onClick={() => fetchData(state.path, true)} />

                <div className={styles.pathBar}>
                    <Input
                        value={inputPath}
                        onChange={(e, data) => setInputPath(data.value)}
                        style={{ flexGrow: 1 }}
                        onKeyDown={(e) => {
                            if (e.key === 'Enter') handleNavigate(inputPath);
                        }}
                    />
                    <Button appearance="primary" onClick={() => handleNavigate(inputPath)}>Go</Button>
                </div>
            </div>

            {state.loading && <ProgressBar />}
            {state.error && <Text style={{ color: 'red' }}>{state.error}</Text>}

            {/* Grid */}
            <div className={styles.gridContainer}>
                <DataGrid
                    items={items}
                    columns={columns}
                    sortable
                    selectionMode="single"
                    onSelectionChange={(e, data) => {
                        // Handle selection
                    }}
                >
                    <DataGridHeader>
                        <DataGridRow>
                            {({ renderHeaderCell }) => (
                                <DataGridHeaderCell>{renderHeaderCell()}</DataGridHeaderCell>
                            )}
                        </DataGridRow>
                    </DataGridHeader>
                    <DataGridBody<FileNode>>
                        {({ item, rowId }) => (
                            <DataGridRow<FileNode>
                                key={rowId}
                                onDoubleClick={() => {
                                    if (item.is_dir) handleNavigate(item.path);
                                }}
                                onKeyDown={(e: React.KeyboardEvent) => {
                                    if ((e.key === 'Enter' || e.key === 'ArrowRight') && item.is_dir) {
                                        handleNavigate(item.path);
                                    }
                                }}
                            >
                                {({ renderCell }) => (
                                    <DataGridCell>{renderCell(item)}</DataGridCell>
                                )}
                            </DataGridRow>
                        )}
                    </DataGridBody >
                </DataGrid >
            </div >

            {/* Status Bar */}
            < div className={styles.statusBar} >
                <Text>{items.length} items</Text>
                <Text>Total Size: {formatSize(state.data?.size || 0)}</Text>
            </div >
        </div >
    );
};

function formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

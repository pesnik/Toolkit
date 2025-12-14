'use client';

import * as React from 'react';
import {
    PieChart, Pie, Cell, Tooltip as RechartsTooltip, Legend, ResponsiveContainer
} from 'recharts';
import { makeStyles, shorthands, Text, tokens } from '@fluentui/react-components';
import { FileNode } from '@/types';

const useStyles = makeStyles({
    container: {
        width: '100%',
        height: '100%',
        display: 'flex',
        flexDirection: 'column',
        alignItems: 'center',
        ...shorthands.padding('10px'),
        backgroundColor: tokens.colorNeutralBackground2,
        ...shorthands.borderRadius('8px'),
    },
    title: {
        marginBottom: '10px',
        fontWeight: 'bold',
    },
    chartContainer: {
        width: '100%',
        flexGrow: 1,
        minHeight: '200px',
    }
});

interface DiskUsageChartProps {
    items: FileNode[];
}

const COLORS = [
    '#0088FE', '#00C49F', '#FFBB28', '#FF8042',
    '#8884d8', '#82ca9d', '#a4de6c', '#d0ed57',
    '#ffc658', '#8dd1e1'
];

export const DiskUsageChart: React.FC<DiskUsageChartProps> = ({ items }) => {
    const styles = useStyles();

    const data = React.useMemo(() => {
        if (!items || items.length === 0) return [];

        // Sort by size desc
        const sorted = [...items].sort((a, b) => b.size - a.size);

        // Take top 8
        const topN = 8;
        const topItems = sorted.slice(0, topN);
        const otherItems = sorted.slice(topN);

        const chartData = topItems.map((item) => ({
            name: item.name,
            value: item.size,
            formattedSize: formatSize(item.size),
        }));

        if (otherItems.length > 0) {
            const otherSize = otherItems.reduce((acc, item) => acc + item.size, 0);
            chartData.push({
                name: 'Others',
                value: otherSize,
                formattedSize: formatSize(otherSize),
            });
        }

        return chartData;
    }, [items]);

    if (data.length === 0) {
        return (
            <div className={styles.container}>
                <Text>No data to visualize</Text>
            </div>
        );
    }

    return (
        <div className={styles.container}>
            <Text className={styles.title} size={400}>Disk Usage Distribution</Text>
            <div className={styles.chartContainer}>
                <ResponsiveContainer width="100%" height="100%">
                    <PieChart>
                        <Pie
                            data={data}
                            cx="50%"
                            cy="50%"
                            labelLine={false}
                            outerRadius="80%"
                            fill="#8884d8"
                            dataKey="value"
                            nameKey="name"
                        >
                            {data.map((entry, index) => (
                                <Cell key={`cell-${index}`} fill={COLORS[index % COLORS.length]} />
                            ))}
                        </Pie>
                        <RechartsTooltip
                            formatter={(value: number, name: string, props: any) => [props?.payload?.formattedSize || value, name]}
                        />
                        <Legend wrapperStyle={{ fontSize: '12px' }} />
                    </PieChart>
                </ResponsiveContainer>
            </div>
        </div>
    );
};

function formatSize(bytes: number): string {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB', 'PB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i];
}

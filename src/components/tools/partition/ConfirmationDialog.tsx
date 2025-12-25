import {
    Dialog,
    DialogSurface,
    DialogBody,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
    Text,
    Input,
    MessageBar,
    MessageBarBody,
    makeStyles,
    tokens,
} from '@fluentui/react-components';
import { useState } from 'react';
import { ErrorCircleRegular } from '@fluentui/react-icons';

const useStyles = makeStyles({
    content: {
        display: 'flex',
        flexDirection: 'column',
        gap: tokens.spacingVerticalL,
    },
    dangerSection: {
        padding: tokens.spacingVerticalL,
        backgroundColor: tokens.colorPaletteRedBackground1,
        borderRadius: tokens.borderRadiusMedium,
        border: `2px solid ${tokens.colorPaletteRedBorder2}`,
    },
    summarySection: {
        padding: tokens.spacingVerticalM,
        backgroundColor: tokens.colorNeutralBackground3,
        borderRadius: tokens.borderRadiusMedium,
    },
    confirmationInput: {
        marginTop: tokens.spacingVerticalM,
    },
});

interface ConfirmationDialogProps {
    open: boolean;
    partitionName: string;
    currentSize: string;
    targetSize: string;
    onConfirm: () => void;
    onCancel: () => void;
}

export function ConfirmationDialog({
    open,
    partitionName,
    currentSize,
    targetSize,
    onConfirm,
    onCancel,
}: ConfirmationDialogProps) {
    const styles = useStyles();
    const [confirmText, setConfirmText] = useState('');

    const isConfirmed = confirmText.toUpperCase() === 'SHRINK';

    const handleConfirm = () => {
        if (isConfirmed) {
            onConfirm();
        }
    };

    return (
        <Dialog open={open} modalType="alert">
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>
                        <div style={{ display: 'flex', alignItems: 'center', gap: tokens.spacingHorizontalS }}>
                            <ErrorCircleRegular style={{ color: tokens.colorPaletteRedForeground1, fontSize: '24px' }} />
                            FINAL CONFIRMATION
                        </div>
                    </DialogTitle>
                    <DialogContent className={styles.content}>
                        <div className={styles.dangerSection}>
                            <Text weight="semibold" size={400} style={{ color: tokens.colorPaletteRedForeground1 }}>
                                ⚠️ THIS OPERATION IS RISKY AND CANNOT BE UNDONE
                            </Text>
                            <Text size={200} style={{ marginTop: tokens.spacingVerticalM }}>
                                Data loss may occur if:
                            </Text>
                            <ul style={{ marginTop: tokens.spacingVerticalXS, marginBottom: 0 }}>
                                <li>Power is lost during operation</li>
                                <li>Process is interrupted</li>
                                <li>Disk has hardware failure</li>
                            </ul>
                        </div>

                        <div className={styles.summarySection}>
                            <Text weight="semibold">Operation Summary:</Text>
                            <div style={{ marginTop: tokens.spacingVerticalS }}>
                                <Text size={200}>Partition: <strong>{partitionName}</strong></Text>
                            </div>
                            <div>
                                <Text size={200}>Current Size: <strong>{currentSize}</strong></Text>
                            </div>
                            <div>
                                <Text size={200}>New Size: <strong>{targetSize}</strong></Text>
                            </div>
                            <div style={{ marginTop: tokens.spacingVerticalS }}>
                                <Text size={200} style={{ color: tokens.colorPaletteRedForeground1 }}>
                                    Space to be removed: <strong>{(parseFloat(currentSize) - parseFloat(targetSize)).toFixed(2)} GB</strong>
                                </Text>
                            </div>
                        </div>

                        <MessageBar intent="error">
                            <MessageBarBody>
                                <strong>FINAL WARNING:</strong> This operation will permanently modify your partition.
                                Make absolutely sure you have a verified backup.
                            </MessageBarBody>
                        </MessageBar>

                        <div className={styles.confirmationInput}>
                            <Text weight="semibold">Type "SHRINK" to confirm:</Text>
                            <Input
                                value={confirmText}
                                onChange={(e) => setConfirmText(e.target.value)}
                                placeholder="Type SHRINK here"
                                style={{ marginTop: tokens.spacingVerticalS }}
                            />
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <Button appearance="secondary" onClick={onCancel}>
                            Cancel
                        </Button>
                        <Button
                            appearance="primary"
                            onClick={handleConfirm}
                            disabled={!isConfirmed}
                            style={{
                                backgroundColor: isConfirmed ? tokens.colorPaletteRedBackground3 : undefined,
                            }}
                        >
                            Start Shrink Operation
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
}

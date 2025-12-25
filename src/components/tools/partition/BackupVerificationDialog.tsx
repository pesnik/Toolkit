import {
    Dialog,
    DialogSurface,
    DialogBody,
    DialogTitle,
    DialogContent,
    DialogActions,
    Button,
    Text,
    Checkbox,
    MessageBar,
    MessageBarBody,
    makeStyles,
    tokens,
} from '@fluentui/react-components';
import { useState } from 'react';
import { WarningRegular } from '@fluentui/react-icons';

const useStyles = makeStyles({
    content: {
        display: 'flex',
        flexDirection: 'column',
        gap: tokens.spacingVerticalL,
    },
    warningSection: {
        padding: tokens.spacingVerticalM,
        backgroundColor: tokens.colorPaletteRedBackground1,
        borderRadius: tokens.borderRadiusMedium,
        borderLeft: `4px solid ${tokens.colorPaletteRedBorder2}`,
    },
    checklistSection: {
        display: 'flex',
        flexDirection: 'column',
        gap: tokens.spacingVerticalM,
    },
    checkboxItem: {
        display: 'flex',
        alignItems: 'flex-start',
        gap: tokens.spacingHorizontalS,
    },
});

interface BackupVerificationDialogProps {
    open: boolean;
    partitionName: string;
    onConfirm: () => void;
    onCancel: () => void;
}

export function BackupVerificationDialog({
    open,
    partitionName,
    onConfirm,
    onCancel,
}: BackupVerificationDialogProps) {
    const styles = useStyles();
    const [checks, setChecks] = useState({
        hasBackup: false,
        verifiedBackup: false,
        recentBackup: false,
        understandsRisk: false,
    });

    const allChecked = Object.values(checks).every((v) => v);

    const handleCheckChange = (key: keyof typeof checks) => {
        setChecks((prev) => ({ ...prev, [key]: !prev[key] }));
    };

    const handleConfirm = () => {
        if (allChecked) {
            onConfirm();
        }
    };

    return (
        <Dialog open={open} modalType="alert">
            <DialogSurface>
                <DialogBody>
                    <DialogTitle>
                        <div style={{ display: 'flex', alignItems: 'center', gap: tokens.spacingHorizontalS }}>
                            <WarningRegular style={{ color: tokens.colorPaletteRedForeground1, fontSize: '24px' }} />
                            Backup Verification Required
                        </div>
                    </DialogTitle>
                    <DialogContent className={styles.content}>
                        <div className={styles.warningSection}>
                            <Text weight="semibold" style={{ color: tokens.colorPaletteRedForeground1 }}>
                                ⚠️ CRITICAL WARNING
                            </Text>
                            <Text size={200} style={{ marginTop: tokens.spacingVerticalS }}>
                                Shrinking partition <strong>{partitionName}</strong> is a RISKY operation that can result in
                                PERMANENT DATA LOSS if:
                            </Text>
                            <ul style={{ marginTop: tokens.spacingVerticalS, marginBottom: 0 }}>
                                <li>Power is lost during the operation</li>
                                <li>The process is interrupted</li>
                                <li>Disk has hardware failures</li>
                                <li>Size calculation is incorrect</li>
                            </ul>
                        </div>

                        <MessageBar intent="error">
                            <MessageBarBody>
                                This operation CANNOT be undone. You MUST have a verified backup before proceeding.
                            </MessageBarBody>
                        </MessageBar>

                        <div className={styles.checklistSection}>
                            <Text weight="semibold">Before proceeding, you MUST confirm:</Text>

                            <div className={styles.checkboxItem}>
                                <Checkbox
                                    checked={checks.hasBackup}
                                    onChange={() => handleCheckChange('hasBackup')}
                                    label="I have backed up ALL important data from this partition"
                                />
                            </div>

                            <div className={styles.checkboxItem}>
                                <Checkbox
                                    checked={checks.verifiedBackup}
                                    onChange={() => handleCheckChange('verifiedBackup')}
                                    label="I have VERIFIED that my backup works and can be restored"
                                />
                            </div>

                            <div className={styles.checkboxItem}>
                                <Checkbox
                                    checked={checks.recentBackup}
                                    onChange={() => handleCheckChange('recentBackup')}
                                    label="The backup was created within the last 24 hours"
                                />
                            </div>

                            <div className={styles.checkboxItem}>
                                <Checkbox
                                    checked={checks.understandsRisk}
                                    onChange={() => handleCheckChange('understandsRisk')}
                                    label="I understand this operation is IRREVERSIBLE and accept the risk of data loss"
                                />
                            </div>
                        </div>
                    </DialogContent>
                    <DialogActions>
                        <Button appearance="secondary" onClick={onCancel}>
                            Cancel
                        </Button>
                        <Button appearance="primary" onClick={handleConfirm} disabled={!allChecked}>
                            I Have Backed Up - Continue
                        </Button>
                    </DialogActions>
                </DialogBody>
            </DialogSurface>
        </Dialog>
    );
}

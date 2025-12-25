import React, { useState } from 'react';
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
  Label,
  Field,
} from '@fluentui/react-components';
import styles from './SpaceInputDialog.module.css';

interface SpaceInputDialogProps {
  open: boolean;
  onClose: () => void;
  onConfirm: (spaceInGB: number) => void;
  partitionName: string;
}

export const SpaceInputDialog: React.FC<SpaceInputDialogProps> = ({
  open,
  onClose,
  onConfirm,
  partitionName,
}) => {
  const [spaceGB, setSpaceGB] = useState<string>('10');

  const handleConfirm = () => {
    const value = parseFloat(spaceGB);
    if (!isNaN(value) && value > 0) {
      onConfirm(value);
    }
  };

  return (
    <Dialog open={open} onOpenChange={(_, data) => !data.open && onClose()}>
      <DialogSurface className={styles.dialog}>
        <DialogBody>
          <DialogTitle>How much space do you need?</DialogTitle>
          <DialogContent className={styles.content}>
            <Text>
              Specify how much additional space you want to add to {partitionName}
            </Text>

            <Field label="Additional Space (GB)">
              <Input
                type="number"
                value={spaceGB}
                onChange={(_, data) => setSpaceGB(data.value)}
                min={1}
                step={1}
              />
            </Field>

            <Text size={200} style={{ marginTop: '8px', opacity: 0.8 }}>
              The wizard will analyze your disk and show you how to free up this space from other partitions.
            </Text>
          </DialogContent>
          <DialogActions>
            <Button appearance="secondary" onClick={onClose}>
              Cancel
            </Button>
            <Button appearance="primary" onClick={handleConfirm}>
              Continue
            </Button>
          </DialogActions>
        </DialogBody>
      </DialogSurface>
    </Dialog>
  );
};

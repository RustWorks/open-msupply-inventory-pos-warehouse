import React from 'react';
import {
  BasicSpinner,
  BasicTextInput,
  InputWithLabelRow,
  Typography,
} from '@common/components';
import { useTranslation } from '@common/intl';
import { Box } from '@openmsupply-client/common';
import { formatPropertyValue } from '../../utils';
import { DraftAsset } from '../../types';
import { useAssets } from '../../api';
import { PropertyInput } from '../../Components/PropertyInput';
interface DetailsProps {
  draft?: DraftAsset;
  onChange: (patch: Partial<DraftAsset>) => void;
}

const Container = ({ children }: { children: React.ReactNode }) => (
  <Box
    display="flex"
    flex={1}
    flexDirection="column"
    alignContent="center"
    padding={4}
  >
    {children}
  </Box>
);

const Section = ({
  children,
  heading,
}: {
  children: React.ReactNode;
  heading: string;
}) => (
  <Box
    display="flex"
    flexDirection="column"
    padding={2}
    paddingRight={4}
    sx={{ maxWidth: '600px', width: '100%' }}
  >
    <Heading>{heading}</Heading>
    {children}
  </Box>
);

const Heading = ({ children }: { children: React.ReactNode }) => (
  <Typography
    sx={{
      marginLeft: '158px',
      fontSize: '20px',
      fontWeight: 'bold',
    }}
  >
    {children}
  </Typography>
);

const Row = ({
  children,
  label,
}: {
  children: React.ReactNode;
  label: string;
}) => (
  <Box paddingTop={1.5}>
    <InputWithLabelRow
      labelWidth="150px"
      label={label}
      labelProps={{
        sx: {
          fontSize: '16px',
          paddingRight: 2,
          textAlign: 'right',
        },
      }}
      Input={
        <Box sx={{}} flex={1}>
          {children}
        </Box>
      }
    />
  </Box>
);

export const Details = ({ draft, onChange }: DetailsProps) => {
  const t = useTranslation('coldchain');

  const { data: assetProperties, isLoading } = useAssets.properties.list({
    assetCategoryId: { equalAnyOrNull: [draft?.assetCategory?.id ?? ''] },
    assetClassId: { equalAnyOrNull: [draft?.assetClass?.id ?? ''] },
    assetTypeId: { equalAnyOrNull: [draft?.assetType?.id ?? ''] },
  });

  if (!draft) return null;

  return (
    <Box display="flex" flex={1}>
      <Container>
        {draft.catalogProperties.length === 0 ? null : (
          <Section heading={t('label.catalogue-properties')}>
            {draft.catalogProperties.map(property => (
              <Row key={property.id} label={property.name}>
                <BasicTextInput
                  value={formatPropertyValue(property, t)}
                  disabled
                  fullWidth
                />
              </Row>
            ))}
          </Section>
        )}
      </Container>
      <Container>
        {isLoading ? <BasicSpinner /> : null}
        {!draft.parsedProperties ? null : (
          <Section heading={t('label.asset-properties')}>
            {assetProperties &&
              assetProperties.nodes.map(property => (
                <Row key={property.key} label={property.name}>
                  <PropertyInput
                    valueType={property.valueType}
                    value={draft.parsedProperties[property.key]}
                    onChange={v =>
                      onChange({
                        parsedProperties: {
                          ...draft.parsedProperties,
                          [property.key]: v ?? null,
                        },
                      })
                    }
                  />
                </Row>
              ))}
          </Section>
        )}
      </Container>
    </Box>
  );
};

use async_graphql::*;
use repository::PackVariantRow;
use service::pack_variant::ItemPackVariant;

pub struct VariantNode {
    pub variant: PackVariantRow,
}

pub struct ItemPackVariantNode {
    pub pack_variants: ItemPackVariant,
}

#[Object]
impl ItemPackVariantNode {
    pub async fn item_id(&self) -> &String {
        &self.pack_variants.item_id
    }

    pub async fn most_used_pack_variant_id(&self) -> &String {
        &self.pack_variants.most_used_pack_variant_id
    }

    pub async fn pack_variants(&self) -> Vec<VariantNode> {
        VariantNode::from_vec(self.pack_variants.pack_variants.clone())
    }
}

impl ItemPackVariantNode {
    pub fn from_domain(pack_variants: ItemPackVariant) -> ItemPackVariantNode {
        ItemPackVariantNode { pack_variants }
    }

    pub fn from_vec(units: Vec<ItemPackVariant>) -> Vec<ItemPackVariantNode> {
        units
            .into_iter()
            .map(ItemPackVariantNode::from_domain)
            .collect()
    }
}

#[Object]
impl VariantNode {
    pub async fn id(&self) -> &String {
        &self.variant.id
    }

    pub async fn short_name(&self) -> &String {
        &self.variant.short_name
    }

    pub async fn long_name(&self) -> &String {
        &self.variant.long_name
    }

    pub async fn pack_size(&self) -> &i32 {
        &self.variant.pack_size
    }
}

impl VariantNode {
    pub fn from_domain(variant: PackVariantRow) -> VariantNode {
        VariantNode { variant }
    }

    pub fn from_vec(units: Vec<PackVariantRow>) -> Vec<VariantNode> {
        units.into_iter().map(VariantNode::from_domain).collect()
    }
}

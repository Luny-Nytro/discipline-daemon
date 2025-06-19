// TODO: Create a Metadata trait that represents a type's metadata
// like its name, its members, the types of its members and thier 
// own metadata.
//
// this trait should be implemented for every type.
//
// Other traits may then use a type's metadata to automatically generate
// an implementation for it, kinda like rust's derive macros.
//
// Here is a basic example of how that would be used:
//
// type ShadowVault = { ... }
//
// const metadata = Metadata.implementForStruct<ShadowVault>({
//   name: "ShadowVault",
//   members: {
//     id: Uuid.metadata,
//     name: Name.metadata,
//     datum: Datum.metadataOptional,
//     protector: CountdownTimer.metadata,
//   }
// })
//
// const displayer = Displayer.autoImplement<ShadowVault>(metadata)
// const jsonSerializer = JsonSerializer.autoImplement<ShadowVault>(metadata)
// const jsonDeserializer = JsonDeserializer.autoImplement<ShadowVault>(metadata)
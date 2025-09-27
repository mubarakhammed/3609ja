// swift-tools-version: 5.7
// The swift-tools-version declares the minimum version of Swift required to build this package.

import PackageDescription

let package = Package(
    name: "NigeriaGeoSDK",
    platforms: [
        .iOS(.v12),
        .macOS(.v10_15),
        .tvOS(.v12),
        .watchOS(.v6)
    ],
    products: [
        .library(
            name: "NigeriaGeoSDK",
            targets: ["NigeriaGeoSDK"]
        ),
    ],
    dependencies: [
        // No external dependencies - pure Swift implementation
    ],
    targets: [
        .target(
            name: "NigeriaGeoSDK",
            dependencies: [],
            path: "Sources/NigeriaGeoSDK",
            resources: [
                .process("Resources")
            ]
        ),
        .testTarget(
            name: "NigeriaGeoSDKTests",
            dependencies: ["NigeriaGeoSDK"],
            path: "Tests/NigeriaGeoSDKTests"
        ),
    ],
    swiftLanguageVersions: [.v5]
)
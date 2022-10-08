using System;
using Microsoft.EntityFrameworkCore.Migrations;

#nullable disable

namespace Spork.Server.Migrations
{
    public partial class Initial : Migration
    {
        protected override void Up(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.CreateTable(
                name: "electorates",
                columns: table => new
                {
                    id = table.Column<Guid>(type: "uuid", nullable: false),
                    name = table.Column<string>(type: "text", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_electorates", x => x.id);
                });

            migrationBuilder.CreateTable(
                name: "members",
                columns: table => new
                {
                    id = table.Column<Guid>(type: "uuid", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_members", x => x.id);
                });

            migrationBuilder.CreateTable(
                name: "member_terms",
                columns: table => new
                {
                    id = table.Column<Guid>(type: "uuid", nullable: false),
                    MemberId = table.Column<Guid>(type: "uuid", nullable: false),
                    ElectorateId = table.Column<Guid>(type: "uuid", nullable: true),
                    date_started = table.Column<DateOnly>(type: "date", nullable: false),
                    start_reason = table.Column<int>(type: "integer", nullable: false),
                    date_ended = table.Column<DateOnly>(type: "date", nullable: false),
                    end_reason = table.Column<int>(type: "integer", nullable: false)
                },
                constraints: table =>
                {
                    table.PrimaryKey("PK_member_terms", x => x.id);
                    table.ForeignKey(
                        name: "FK_member_terms_electorates_ElectorateId",
                        column: x => x.ElectorateId,
                        principalTable: "electorates",
                        principalColumn: "id");
                    table.ForeignKey(
                        name: "FK_member_terms_members_MemberId",
                        column: x => x.MemberId,
                        principalTable: "members",
                        principalColumn: "id",
                        onDelete: ReferentialAction.Cascade);
                });

            migrationBuilder.CreateIndex(
                name: "IX_member_terms_ElectorateId",
                table: "member_terms",
                column: "ElectorateId");

            migrationBuilder.CreateIndex(
                name: "IX_member_terms_MemberId",
                table: "member_terms",
                column: "MemberId");
        }

        protected override void Down(MigrationBuilder migrationBuilder)
        {
            migrationBuilder.DropTable(
                name: "member_terms");

            migrationBuilder.DropTable(
                name: "electorates");

            migrationBuilder.DropTable(
                name: "members");
        }
    }
}

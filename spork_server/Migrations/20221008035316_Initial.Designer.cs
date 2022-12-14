// <auto-generated />
using System;
using Microsoft.EntityFrameworkCore;
using Microsoft.EntityFrameworkCore.Infrastructure;
using Microsoft.EntityFrameworkCore.Migrations;
using Microsoft.EntityFrameworkCore.Storage.ValueConversion;
using Npgsql.EntityFrameworkCore.PostgreSQL.Metadata;
using Spork.Server.Persistence;

#nullable disable

namespace Spork.Server.Migrations
{
    [DbContext(typeof(SporkDbContext))]
    [Migration("20221008035316_Initial")]
    partial class Initial
    {
        protected override void BuildTargetModel(ModelBuilder modelBuilder)
        {
#pragma warning disable 612, 618
            modelBuilder
                .HasAnnotation("ProductVersion", "6.0.9")
                .HasAnnotation("Relational:MaxIdentifierLength", 63);

            NpgsqlModelBuilderExtensions.UseIdentityByDefaultColumns(modelBuilder);

            modelBuilder.Entity("Spork.Server.Persistence.ParliamentElectorate", b =>
                {
                    b.Property<Guid>("Id")
                        .ValueGeneratedOnAdd()
                        .HasColumnType("uuid")
                        .HasColumnName("id");

                    b.Property<string>("Name")
                        .IsRequired()
                        .HasColumnType("text")
                        .HasColumnName("name");

                    b.HasKey("Id");

                    b.ToTable("electorates");
                });

            modelBuilder.Entity("Spork.Server.Persistence.ParliamentMember", b =>
                {
                    b.Property<Guid>("Id")
                        .ValueGeneratedOnAdd()
                        .HasColumnType("uuid")
                        .HasColumnName("id");

                    b.HasKey("Id");

                    b.ToTable("members");
                });

            modelBuilder.Entity("Spork.Server.Persistence.ParliamentMemberTerm", b =>
                {
                    b.Property<Guid>("Id")
                        .ValueGeneratedOnAdd()
                        .HasColumnType("uuid")
                        .HasColumnName("id");

                    b.Property<Guid?>("ElectorateId")
                        .HasColumnType("uuid");

                    b.Property<int>("EndReason")
                        .HasColumnType("integer")
                        .HasColumnName("end_reason");

                    b.Property<DateOnly>("Ended")
                        .HasColumnType("date")
                        .HasColumnName("date_ended");

                    b.Property<Guid>("MemberId")
                        .HasColumnType("uuid");

                    b.Property<int>("StartReason")
                        .HasColumnType("integer")
                        .HasColumnName("start_reason");

                    b.Property<DateOnly>("Started")
                        .HasColumnType("date")
                        .HasColumnName("date_started");

                    b.HasKey("Id");

                    b.HasIndex("ElectorateId");

                    b.HasIndex("MemberId");

                    b.ToTable("member_terms");
                });

            modelBuilder.Entity("Spork.Server.Persistence.ParliamentMemberTerm", b =>
                {
                    b.HasOne("Spork.Server.Persistence.ParliamentElectorate", "Electorate")
                        .WithMany()
                        .HasForeignKey("ElectorateId");

                    b.HasOne("Spork.Server.Persistence.ParliamentMember", "Member")
                        .WithMany("Terms")
                        .HasForeignKey("MemberId")
                        .OnDelete(DeleteBehavior.Cascade)
                        .IsRequired();

                    b.Navigation("Electorate");

                    b.Navigation("Member");
                });

            modelBuilder.Entity("Spork.Server.Persistence.ParliamentMember", b =>
                {
                    b.Navigation("Terms");
                });
#pragma warning restore 612, 618
        }
    }
}
